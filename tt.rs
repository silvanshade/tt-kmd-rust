// SPDX-License-Identifier: GPL-2.0

//! Tenstorrent device driver library.

pub(crate) mod driver;
pub(crate) mod hw;
pub(crate) mod version;

pub(crate) use self::version::version;
