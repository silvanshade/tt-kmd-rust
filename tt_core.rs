// SPDX-License-Identifier: GPL-2.0

use ::kernel::prelude::*;

mod tt;

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
