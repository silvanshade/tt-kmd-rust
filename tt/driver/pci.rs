// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

#[allow(clippy::empty_structs_with_brackets, reason = "rust-for-linux")]
#[pin_data]
pub struct Driver {}

::kernel::pci_device_table!(
    PCI_TABLE,
    MODULE_PCI_TABLE,
    <Driver as ::kernel::pci::Driver>::IdInfo,
    [
        crate::tt::hw::HWCONFIG_BLACKHOLE.pci_device_table_id(),
        crate::tt::hw::HWCONFIG_GRAYSKULL.pci_device_table_id(),
        crate::tt::hw::HWCONFIG_WORMHOLE.pci_device_table_id(),
    ]
);

impl ::kernel::pci::Driver for Driver {
    type IdInfo = crate::tt::hw::HwConfig;

    const ID_TABLE: ::kernel::pci::IdTable<Self::IdInfo> = &PCI_TABLE;

    fn probe(dev: &::kernel::pci::Device<::kernel::device::Core>, id_info: &Self::IdInfo) -> Result<Pin<KBox<Self>>> {
        let version = crate::tt::version()?;
        let version = version.to_str()?;
        pr_info!("(init): {}\n", version);
        pr_info!(
            "(probe): vendor_id={:#06X}, device_id={:#06X}, name={}\n",
            dev.vendor_id(),
            dev.device_id(),
            id_info.name,
        );
        dev.enable_device_mem()?;
        dev.set_master();
        let init = try_pin_init!(Self {});
        let this = KBox::pin_init(init, GFP_KERNEL)?;
        Ok(this)
    }
}
