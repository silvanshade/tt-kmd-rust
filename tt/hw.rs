// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

pub(crate) mod blackhole;
pub(crate) mod grayskull;
pub(crate) mod wormhole;

pub(crate) use blackhole::HWCONFIG_BLACKHOLE;
pub(crate) use grayskull::HWCONFIG_GRAYSKULL;
pub(crate) use wormhole::HWCONFIG_WORMHOLE;

pub(crate) const PCI_VENDOR_ID_TENSTORRENT: u32 = 0x1E52;

#[allow(unreachable_pub, reason = "rust-for-linux")]
pub struct HwConfig {
    pub vendor_id: u32,
    pub device_id: u32,
    pub name: &'static CStr,
}

impl HwConfig {
    pub(crate) const fn device_id(&self) -> ::kernel::pci::DeviceId {
        let vendor = self.vendor_id;
        let device = self.device_id;
        ::kernel::pci::DeviceId::from_id(vendor, device)
    }

    pub(crate) const fn pci_device_table_id(self) -> (::kernel::pci::DeviceId, Self) {
        (self.device_id(), self)
    }
}
