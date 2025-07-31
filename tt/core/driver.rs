// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

#[pin_data]
pub struct TtCore {}

::kernel::pci_device_table!(
    PCI_TABLE,
    MODULE_PCI_TABLE,
    <TtCore as ::kernel::pci::Driver>::IdInfo,
    [
        crate::tt::hw::HWCONFIG_BLACKHOLE.pci_device_table_id(),
        crate::tt::hw::HWCONFIG_GRAYSKULL.pci_device_table_id(),
        crate::tt::hw::HWCONFIG_WORMHOLE.pci_device_table_id(),
    ]
);

impl ::kernel::pci::Driver for TtCore {
    type IdInfo = crate::tt::hw::HwConfig;

    const ID_TABLE: ::kernel::pci::IdTable<Self::IdInfo> = &PCI_TABLE;

    fn probe(pdev: &::kernel::pci::Device<::kernel::device::Core>, info: &Self::IdInfo) -> Result<Pin<KBox<Self>>> {
        pr_info!(
            "(probe): vendor_id={:#06X}, device_id={:#06X}\n",
            pdev.vendor_id(),
            pdev.device_id(),
        );
        pdev.enable_device_mem()?;
        pdev.set_master();
        let init = try_pin_init!(Self {});
        let this = KBox::pin_init(init, GFP_KERNEL)?;
        Ok(this)
    }
}
