// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

/// Definition related to the Tenstorrent Blackhole hardware.
pub(crate) mod blackhole;

/// Definitions related to the Tenstorrent Grayskull hardware.
pub(crate) mod grayskull;

/// Definitions related to the Tenstorrent Wormhole hardware.
pub(crate) mod wormhole;

pub(crate) use blackhole::HWCONFIG_BLACKHOLE;
pub(crate) use grayskull::HWCONFIG_GRAYSKULL;
pub(crate) use wormhole::HWCONFIG_WORMHOLE;

/// The Tenstorrent PCI vendor string.
pub(crate) const PCI_VENDOR_ID_TENSTORRENT: u32 = 0x1E52;

/// A hardware configuration description.
#[allow(unreachable_pub, reason = "rust-for-linux")]
pub struct HwConfig {
    /// The PCI vendor ID.
    pub vendor_id: u32,
    /// The PCI device ID.
    pub device_id: u32,
    /// The hardware device name.
    pub name: &'static CStr,
}

impl HwConfig {
    /// Returns the PCI device ID.
    pub(crate) const fn device_id(&self) -> ::kernel::pci::DeviceId {
        let vendor = self.vendor_id;
        let device = self.device_id;
        ::kernel::pci::DeviceId::from_id(vendor, device)
    }

    /// Returns the PCI device table entry.
    pub(crate) const fn pci_device_table_item(self) -> (::kernel::pci::DeviceId, Self) {
        (self.device_id(), self)
    }
}
