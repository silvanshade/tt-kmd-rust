// SPDX-License-Identifier: GPL-2.0

use ::kernel::{device, pci, prelude::*};

mod tt;

pub(crate) const MODULE_NAME: &kernel::str::CStr = <LocalModule as kernel::ModuleMetadata>::NAME;

::kernel::module_pci_driver! {
    type: tt::core::driver::TtCore,
    name: "tt_core",
    authors: ["Darin Morrison"],
    description: "tenstorrent driver (rust)",
    license: "GPL",
    params: {
        auto_reset_timeout: usize {
            default: 10,
            description: "Timeout duration in seconds for M3 auto reset to occur.",
        },
        dma_address_bits: usize {
            default: 0,
            description: "DMA address bits, 0 for automatic.",
        },
        max_devices: usize {
            default: 32,
            description: "Maximum number of tenstorrent devices (chips) to support.",
        },
        reset_limit: usize {
            default: 10,
            description: "Maximum number of times to reset device during boot.",
        },
    },
}
