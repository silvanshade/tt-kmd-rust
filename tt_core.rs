// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

mod tt;

module! {
    type: Tenstorrent,
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

struct Tenstorrent {}

impl ::kernel::Module for Tenstorrent {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let version = tt::version::version()?;
        let version = version.to_str()?;
        pr_info!("(init)\n");
        pr_info!("v{version}");
        Ok(Tenstorrent {})
    }
}

impl Drop for Tenstorrent {
    fn drop(&mut self) {
        pr_info!("(exit)\n");
    }
}
