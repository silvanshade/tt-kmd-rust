// SPDX-License-Identifier: GPL-2.0

use ::kernel::{prelude::*, str::CString};

/// The module major version.
const MAJOR_VERSION: usize = 0;

/// The module minor version.
const MINOR_VERSION: usize = 0;

/// The module patch version.
const PATCH_VERSION: usize = 0;

/// Returns the expanded version string.
///
/// # Errors
///
/// * Errors if formatting fails.
pub(crate) fn version() -> Result<CString> {
    CString::try_from_fmt(fmt!("{MAJOR_VERSION}.{MINOR_VERSION}.{PATCH_VERSION}"))
}
