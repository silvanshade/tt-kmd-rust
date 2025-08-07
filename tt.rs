// SPDX-License-Identifier: GPL-2.0

//! Tenstorrent device driver library.

/// Definitions related to the hardware devices and their registration.
pub(crate) mod device;

/// Definitions related to Tenstorrent hardware configurations.
pub(crate) mod hw;

/// Definitions related to the module version metadata.
pub(crate) mod version;

pub(crate) use self::version::version;
