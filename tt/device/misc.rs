// SPDX-License-Identifier: GPL-2.0

use ::kernel::{
    miscdevice::{MiscDeviceOptions, MiscDeviceRegistration},
    prelude::*,
    types::ARef,
};

pub(crate) use self::misc::MiscOrdinal;

/// A misc device reference.
#[allow(clippy::empty_structs_with_brackets, reason = "rust-for-linux")]
#[pin_data(PinnedDrop)]
pub(crate) struct TtMisc {
    #[pin]
    inner: ::kernel::sync::Mutex<i32>,
    dev: ARef<::kernel::device::Device>,
}

#[vtable]
impl ::kernel::miscdevice::MiscDevice for TtMisc {
    type Ptr = Pin<KBox<Self>>;

    fn open(
        _file: &::kernel::fs::File,
        misc: &::kernel::miscdevice::MiscDeviceRegistration<Self>,
    ) -> Result<Self::Ptr> {
        let dev = ARef::from(misc.device());
        dev_info!(dev, "opening\n");
        KBox::try_pin_init(
            try_pin_init! {
                TtMisc {
                    inner <- ::kernel::new_mutex!(0i32),
                    dev,
                }
            },
            GFP_KERNEL,
        )
    }
}

impl TtMisc {
    /// The devfs prefix for the misc device.
    const DEVICE_PREFIX: &CStr = ::kernel::c_str!("tenstorrent");

    /// Registers a misc device and returns the ordinal and registration.
    ///
    /// # Errors
    ///
    /// * Errors if obtaining the next ordinal/name pair fails.
    pub(crate) fn register() -> Result<(MiscOrdinal, impl PinInit<MiscDeviceRegistration<Self>, Error>)> {
        let (ordinal, name) = self::supply::MISC_SUPPLY.next().inspect_err(|err| {
            if err == &ENOSPC {
                pr_err!("Cannot create misc device: `max_devices` already allocated\n");
            }
        })?;
        let options = MiscDeviceOptions { name };
        let reg = MiscDeviceRegistration::register(options);
        Ok((ordinal, reg))
    }
}

#[pinned_drop]
impl PinnedDrop for TtMisc {
    fn drop(self: Pin<&mut Self>) {}
}

#[allow(clippy::module_inception, reason = "style")]
mod misc {
    /// A wrapper for a misc ordinal which frees it from the pool on drop.
    pub(crate) struct MiscOrdinal {
        /// The value of the ordinal.
        value: usize,
    }

    impl MiscOrdinal {
        /// Creates a [`MiscOrdinal`].
        pub(crate) const fn new(value: usize) -> Self {
            Self { value }
        }
    }

    impl Drop for MiscOrdinal {
        fn drop(&mut self) {
            let ordinal = self.value;
            ::kernel::pr_info!("freeing ordinal: {ordinal}\n");
            super::supply::MISC_SUPPLY.free(ordinal);
        }
    }
}

/// Assignment of misc device ordinals and devfs names.
mod supply {
    use ::kernel::{prelude::*, sync::SetOnce};

    use crate::tt::device::misc::MiscOrdinal;

    /// The global static supply.
    pub(super) static MISC_SUPPLY: MiscSupply = MiscSupply::new();

    /// A supply for assignment of misc device ordinals and devfs names.
    pub(super) struct MiscSupply {
        /// The pool wrapped in [`SetOnce`] to allow static initialization.
        once: SetOnce<Pin<KBox<super::private::MiscPool>>>,
    }

    // SAFETY: API only allows mutations through [Mutex].
    unsafe impl Sync for MiscSupply {}

    impl MiscSupply {
        /// Creates a [`MiscSupply`].
        pub(super) const fn new() -> Self {
            let once = SetOnce::new();
            Self { once }
        }

        /// Removes the ordinal from the supply and frees it for subsequent use.
        pub(super) fn free(&self, ordinal: usize) {
            if let Some(pool) = self.once.as_ref() {
                pool.free(ordinal);
            }
        }

        /// Returns the next misc device ordinal and the associated devfs name.
        ///
        /// # Errors
        ///
        /// * Errors if initializing the [`MiscSupply`] fails.
        /// * Errors if updating the [`SetOnce`] fails.
        pub(super) fn next(&self) -> Result<(MiscOrdinal, &'static CStr)> {
            self.init()?;
            self.once.as_ref().ok_or(ENOMEM)?.next()
        }

        /// Initialize the misc supply if not already initialized.
        ///
        /// # Errors
        ///
        /// * Errors if allocating the [`MiscPool`] fails.
        fn init(&self) -> Result<()> {
            // Return early if initalized.
            if self.once.as_ref().is_some() {
                return Ok(());
            }

            // Otherwise populate the [SetOnce].
            let max = crate::module_parameters::max_devices.value();
            let init = super::private::MiscPool::new(*max);
            let kbox = KBox::pin_init(init, GFP_KERNEL)?;
            self.once.populate(kbox);

            Ok(())
        }
    }
}

/// Private APIS for this module.
mod private {
    #![allow(clippy::impl_trait_in_params, reason = "pin_data macro")]

    use ::core::num::NonZero;
    use ::kernel::{
        prelude::*,
        str::CString,
        xarray::{self, XArray},
    };

    use crate::tt::device::misc::MiscOrdinal;

    /// A pool for assignment misc device ordinals and devfs names.
    #[pin_data]
    pub(super) struct MiscPool {
        /// The maximum number of ordinals to choose from.
        limit: NonZero<u32>,
        /// The underlying [XArray] providing the pool API.
        #[pin]
        names: XArray<KBox<CString>>,
    }

    impl MiscPool {
        /// Creates a pool for assigning misc device ordinals and devfs names.
        pub(super) fn new(mut limit: u32) -> impl PinInit<Self> {
            limit += 1;
            // SAFETY: Value of `max + 1` is non-zero by definition.
            let limit = unsafe { NonZero::new_unchecked(limit) };
            pin_init!(Self {
                limit,
                names <- XArray::new(xarray::AllocKind::Alloc),
            })
        }

        /// Returns the next misc device ordinal and the associated devfs name.
        ///
        /// # Errors
        ///
        /// * Errors if reserving an ordinal fails (if all are exhausted).
        /// * Errors if formatting the name from the ordinal fails.
        /// * Errors if allocating the name for the reservation fails.
        /// * Errors if filling the reservation fails.
        /// * Errors if looking up the [`&CStr`] from the [`XArray`] fails.
        pub(super) fn next(&self) -> Result<(MiscOrdinal, &'static CStr)> {
            // Find the next available reservation.
            let mut guard = self.names.lock();
            let res = guard.reserve_limit(.. self.limit.get(), GFP_KERNEL)?;

            // Compute the prefix and get the ordinal from the reservation.
            let prefix = crate::tt::device::misc::TtMisc::DEVICE_PREFIX;
            let ordinal = res.index();

            // Fill the reservation with the computed devfs name.
            let value = CString::try_from_fmt(fmt!("{prefix}/{ordinal}"))?;
            let value = KBox::new(value, GFP_KERNEL)?;
            res.fill_locked(&mut guard, value)?;

            // Construct the [&'static CStr] from the computed name.
            let data = guard.get(ordinal).ok_or(ERANGE)?.as_char_ptr();
            let ordinal = MiscOrdinal::new(ordinal);
            // SAFETY: names are never dropped until module is unloaded.
            let name = unsafe { CStr::from_char_ptr(data) };

            // Return the obtained ordinal and computed name.
            Ok((ordinal, name))
        }

        /// Removes the ordinal from the pool and frees it for subsequent use.
        pub(super) fn free(&self, ordinal: usize) {
            self.names.lock().remove(ordinal);
        }
    }
}
