// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

#[pin_data]
pub struct TtCore {}

::kernel::pci_device_table!(
    PCI_TABLE,
    MODULE_PCI_TABLE,
    <TtCore as ::kernel::pci::Driver>::IdInfo,
    [
        (
            ::kernel::pci::DeviceId::from_id(
                crate::tt::device::PCI_VENDOR_ID_TENSTORRENT,
                crate::tt::device::PCI_DEVICE_ID_GRAYSKULL,
            ),
            ()
        ),
        (
            ::kernel::pci::DeviceId::from_id(
                crate::tt::device::PCI_VENDOR_ID_TENSTORRENT,
                crate::tt::device::PCI_DEVICE_ID_WORMHOLE,
            ),
            ()
        ),
        (
            ::kernel::pci::DeviceId::from_id(
                crate::tt::device::PCI_VENDOR_ID_TENSTORRENT,
                crate::tt::device::PCI_DEVICE_ID_BLACKHOLE,
            ),
            ()
        )
    ]
);

impl ::kernel::pci::Driver for TtCore {
    type IdInfo = ();

    const ID_TABLE: ::kernel::pci::IdTable<Self::IdInfo> = &PCI_TABLE;

    fn probe(pdev: &::kernel::pci::Device<::kernel::device::Core>, _info: &Self::IdInfo) -> Result<Pin<KBox<Self>>> {
        pr_info!(
            "(probe): vendor_id={:#06X}, device_id={:#06X}\n",
            pdev.vendor_id(),
            pdev.device_id()
        );
        pdev.enable_device_mem()?;
        pdev.set_master();
        let init = try_pin_init!(Self {});
        let this = KBox::pin_init(init, GFP_KERNEL)?;
        Ok(this)
    }
}
