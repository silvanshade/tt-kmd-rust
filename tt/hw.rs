// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

pub mod blackhole;
pub mod grayskull;
pub mod wormhole;

pub use blackhole::HWCONFIG_BLACKHOLE;
pub use grayskull::HWCONFIG_GRAYSKULL;
pub use wormhole::HWCONFIG_WORMHOLE;

pub const PCI_VENDOR_ID_TENSTORRENT: u32 = 0x1E52;

pub struct HwConfig {
    pub vendor_id: u32,
    pub device_id: u32,
    pub name: &'static CStr,
}

impl HwConfig {
    pub const fn device_id(&self) -> ::kernel::pci::DeviceId {
        let vendor = self.vendor_id;
        let device = self.device_id;
        ::kernel::pci::DeviceId::from_id(vendor, device)
    }

    pub const fn pci_device_table_id(self) -> (::kernel::pci::DeviceId, HwConfig) {
        (self.device_id(), self)
    }
}
