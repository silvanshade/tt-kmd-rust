// SPDX-License-Identifier: GPL-2.0

use ::kernel::{prelude::*, str::CString};

const MAJOR_VERSION: usize = 0;
const MINOR_VERSION: usize = 0;
const PATCH_VERSION: usize = 0;

pub(crate) fn version() -> Result<CString> {
    CString::try_from_fmt(fmt!("{MAJOR_VERSION}.{MINOR_VERSION}.{PATCH_VERSION}"))
}
