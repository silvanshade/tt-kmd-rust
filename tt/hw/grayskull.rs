// SPDX-License-Identifier: GPL-2.0

use ::kernel::c_str;

use crate::tt::hw::HwConfig;

/// The Tenstorrent Grayskull hardware configuration description.
pub(crate) const HWCONFIG_GRAYSKULL: HwConfig = HwConfig {
    vendor_id: crate::tt::hw::PCI_VENDOR_ID_TENSTORRENT,
    device_id: 0xFACA,
    name: c_str!("Grayskull"),
};
