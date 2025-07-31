// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

module! {
    type: Tenstorrent,
    name: "tt_core",
    authors: ["Darin Morrison"],
    description: "tenstorrent driver (rust)",
    license: "GPL",
}

struct Tenstorrent {}

impl ::kernel::Module for Tenstorrent {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("(init)\n");
        Ok(Tenstorrent {})
    }
}

impl Drop for Tenstorrent {
    fn drop(&mut self) {
        pr_info!("(exit)\n");
    }
}
