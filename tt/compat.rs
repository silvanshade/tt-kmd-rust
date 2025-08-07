// SPDX-License-Identifier: GPL-2.0

use ::core::ptr::NonNull;
use ::kernel::{
    bindings::{self, attribute, attribute_group, kobj_attribute, kobject},
    prelude::*,
    types::Opaque,
};

/// Configuration state for compatibility with official driver.
///
/// This structure is responsible for creating an overlay in sysfs and elsewhere
/// for compatibility with the official driver with respect to userland
/// applications that may check for files like
/// `/sys/module/tenstorrent/version`.
///
/// NOTE: This is primarily intended to facilitate easier development and
/// testing with existing userspace applications. In the long run, this
/// compatibility layer will likely be removed.
#[derive(Default)]
pub(crate) struct TenstorrentCompat {
    /// The optionally initialized compatibility overlay.
    #[allow(unused, reason = "resident kobject")]
    overlay: Option<Overlay>,
}

impl TenstorrentCompat {
    /// Creates a new [`TenstorrentCompat`].
    ///
    /// This involves checking for the existence of `/sys/module/tenstorrent`
    /// and, if it does not exist, installing a compatibility overlay at that
    /// path by creating a corresponding [`kobject`].
    ///
    /// The purpose of this compatibility overlay is for easier integration with
    /// existing userspace tools, so that they can be used without significant
    /// modification.
    ///
    /// NOTE: This feature will likely go away in the long term so it is advised
    /// to use with caution and to not rely on it being the expected behavior.
    ///
    /// # Errors
    ///
    /// * Errors if [`Self::should_enable_overlay`] fails.
    ///
    /// [`kobject`]: ::kernel::uapi::kobject
    pub(crate) fn new(module: &'static ThisModule) -> Result<Self> {
        let overlay = Self::should_enable_overlay(module)?
            .then(|| Overlay::new(module))
            .transpose()
            .or_else(|err| {
                if Self::sys_module_tenstorrent_exists(module)? {
                    Ok(None)
                } else {
                    Err(err)
                }
            })?;
        Ok(Self { overlay })
    }

    /// Returns `true` if the compatibility overlay should be enabled.
    ///
    /// # Errors
    ///
    /// * Errors if [`Self::sys_module_tenstorrent_exists`] fails.
    fn should_enable_overlay(this: &'static ThisModule) -> Result<bool> {
        if 0 == *crate::module_parameters::unsafe_compatibility_overlay.value() {
            return Ok(false);
        }
        pr_warn!("unsafe_compatibility_overlay=1");
        if Self::sys_module_tenstorrent_exists(this)? {
            pr_warn!("disabling compatibility overlay; tenstorrent module already loaded");
            return Ok(false);
        }
        Ok(true)
    }

    /// Returns `Ok(true)` if `/sys/module/tenstorrent` exists.
    ///
    /// # Errors
    ///
    /// * Errors if the path in question is found (by a call to `filp_open`) but
    ///   the subsequent call to `filp_close` somehow fails.
    fn sys_module_tenstorrent_exists(this: &'static ThisModule) -> Result<bool> {
        let this = this.as_ptr();

        // SAFETY: `this` is a valid pointer from the kernel.
        let mkobj = unsafe { *this }.mkobj;
        let kset = mkobj.kobj.kset;
        let Some(kset) = NonNull::new(kset) else {
            return Err(EINVAL);
        };

        if let Some(kobj) = {
            let kset = kset.as_ptr();
            let name = Overlay::MODULE_NAME.as_ptr().cast();
            // SAFETY: `kset` is a valid pointer from the kernel.
            let kobj = unsafe { bindings::kset_find_obj(kset, name) };
            NonNull::new(kobj)
        } {
            // SAFETY: `kobj` was just validated as non-NULL.
            unsafe { bindings::kobject_put(kobj.as_ptr()) };
            return Ok(true);
        }

        Ok(false)
    }
}

/// The overlay structure for sysfs compatibility with the official Tenstorrent
/// kernel-mode driver.
pub(crate) struct Overlay {
    /// The kobject for the `/sys/module/tenstorrent` sysfs entry.
    kobj: NonNull<kobject>,
}

/// SAFETY: [`Overlay`] does not offer Rust access to non-[`Send`] internals
/// throughout it's lifetime.
unsafe impl Send for Overlay {}

/// SAFETY: [`Overlay`] does not offer Rust access to non-[`Sync`] internals
/// throughout it's lifetime.
unsafe impl Sync for Overlay {}

impl Drop for Overlay {
    fn drop(&mut self) {
        let grp = Opaque::raw_get(&raw const ATTR_GROUP);
        // SAFETY: `kobj` and `grp` were only processed through Linux APIs.
        unsafe { bindings::sysfs_remove_group(self.kobj.as_ptr(), grp) };
        // SAFETY: `kobj` was only processed through Linux APIs.
        unsafe { bindings::kobject_put(self.kobj.as_ptr()) };
    }
}

impl Overlay {
    /// The name for the compatibility overlay.
    const MODULE_NAME: &CStr = ::kernel::c_str!("tenstorrent");
    /// The module version for the compatibility overlay.
    const MODULE_VERSION: &CStr = ::kernel::c_str!("2.3.0");

    /// Constructs a new [`Overlay`].
    ///
    /// # Errors
    ///
    /// * Errors if [`Self::sys_module_tenstorrent_create`] fails.
    /// * Errors if `sysfs_create_group` fails.
    fn new(this: &'static ThisModule) -> Result<Self> {
        let kobj = Self::sys_module_tenstorrent_create(this)?;
        let attr_group = Opaque::raw_get(&raw const ATTR_GROUP);

        // SAFETY: Arguments are valid for API. Error is propagated on failure.
        let err = unsafe { bindings::sysfs_create_group(kobj.as_ptr(), attr_group) };
        ::kernel::error::to_result(err).inspect_err(|_err| {
            // SAFETY: `kobj` was created by `kobject_create_and_add`.
            unsafe { bindings::kobject_put(kobj.as_ptr()) };
        })?;

        pr_warn!("created tenstorrent compatibility overlay\n");

        Ok(Self { kobj })
    }

    /// # Errors
    ///
    /// * Errors if `kobject_create_and_add` returns `NULL`.
    fn sys_module_tenstorrent_create(this: &'static ThisModule) -> Result<NonNull<kobject>> {
        let this = this.as_ptr();

        // SAFETY: The pointer is valid since it came from a reference.
        let sys_modules_this = unsafe { *this }.mkobj;
        let sys_modules = sys_modules_this.kobj.parent;

        // SAFETY: The name is in kernel-space.
        let sys_modules_that = unsafe { bindings::kobject_create_and_add(Self::MODULE_NAME.as_ptr(), sys_modules) };
        NonNull::new(sys_modules_that).ok_or(EIO)
    }
}

/// The `version` attribute for `/sys/module/tenstorrent`.
///
/// NOTE: We use `Opaque` to prevent implicit copies.
static mut ATTR_VERSION: Opaque<kobj_attribute> = {
    extern "C" fn show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut ::kernel::ffi::c_char) -> isize {
        let fmt = ::kernel::c_str!("%s\n").as_char_ptr();
        let version = Overlay::MODULE_VERSION;
        // SAFETY: Arguments are valid for API. Error is propagated on failure.
        let res = unsafe { bindings::sysfs_emit(buf, fmt, version) };
        #[allow(clippy::as_conversions, reason = "kernel returned value")]
        let res = res as isize;
        res
    }

    Opaque::new(kobj_attribute {
        attr: attribute {
            name: c"version".as_ptr().cast(),
            mode: 0o444,
        },
        show: Some(show),
        store: None,
    })
};

/// The attributes for `/sys/module/tenstorrent`.
const ATTRS: [*mut attribute; 2] = [
    {
        let attr_version = Opaque::raw_get(&raw const ATTR_VERSION);
        // SAFETY: `attr_version` is safe for dereference by construction above.
        unsafe { &raw mut (*attr_version).attr }
    },
    ::core::ptr::null_mut(),
];

/// The attribute group for `/sys/module/tenstorrent`.
///
/// NOTE: We use `Opaque` to prevent implicit copies.
static mut ATTR_GROUP: Opaque<attribute_group> = {
    let this = Opaque::<attribute_group>::uninit();
    // SAFETY: This idiom is safe for `uninit` and doesn't create temporaries.
    let attrs = unsafe { &raw mut (*this.get()).attrs };
    // SAFETY: `write` is safe for `uninit` and `attrs` is not aliased.
    unsafe { attrs.write(ATTRS.as_ptr().cast_mut()) };
    this
};
