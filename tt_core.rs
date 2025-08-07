// SPDX-License-Identifier: GPL-2.0

// nightly features
#![cfg_attr(feature = "nightly", feature(strict_provenance_lints))]
#![cfg_attr(feature = "nightly", warn(fuzzy_provenance_casts))]
#![cfg_attr(feature = "nightly", warn(lossy_provenance_casts))]
// deny warnings
#![deny(warnings)]
// deny most clippy warnings
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::restriction)]
// allow specific clippy warnings
#![allow(clippy::absolute_paths, reason = "style")]
#![allow(clippy::allow_attributes, reason = "style")]
#![allow(clippy::arbitrary_source_item_ordering, reason = "style")]
#![allow(clippy::arithmetic_side_effects, reason = "style")]
#![allow(clippy::blanket_clippy_restriction_lints, reason = "style")]
#![allow(clippy::cargo_common_metadata, reason = "style")]
#![allow(clippy::equatable_if_let, reason = "style")]
#![allow(clippy::implicit_return, reason = "style")]
#![allow(clippy::indexing_slicing, reason = "style")]
#![allow(clippy::inline_always, reason = "style")]
#![allow(clippy::integer_division_remainder_used, reason = "style")]
#![allow(clippy::integer_division, reason = "style")]
#![allow(clippy::match_ref_pats, reason = "style")]
#![allow(clippy::min_ident_chars, reason = "style")]
#![allow(clippy::missing_inline_in_public_items, reason = "style")]
#![allow(clippy::missing_trait_methods, reason = "style")]
#![allow(clippy::module_name_repetitions, reason = "style")]
#![allow(clippy::multiple_crate_versions, reason = "upstream")]
#![allow(clippy::multiple_inherent_impl, reason = "style")]
#![allow(clippy::needless_borrowed_reference, reason = "style")]
#![allow(clippy::partial_pub_fields, reason = "style")]
#![allow(clippy::pub_with_shorthand, reason = "style")]
#![allow(clippy::question_mark_used, reason = "style")]
#![allow(clippy::ref_patterns, reason = "style")]
#![allow(clippy::self_named_module_files, reason = "style")]
#![allow(clippy::semicolon_inside_block, reason = "style")]
#![allow(clippy::semicolon_outside_block, reason = "style")]
#![allow(clippy::shadow_reuse, reason = "style")]
#![allow(clippy::shadow_same, reason = "style")]
#![allow(clippy::shadow_unrelated, reason = "style")]
#![allow(clippy::similar_names, reason = "style")]
#![allow(clippy::single_call_fn, reason = "style")]
#![allow(clippy::unseparated_literal_suffix, reason = "style")]
// rust-for-linux
#![allow(clippy::expl_impl_clone_on_copy, reason = "rust-for-linux")]
#![allow(clippy::mem_forget, reason = "rust-for-linux")]
#![allow(clippy::negative_feature_names, reason = "rust-for-linux")]
#![allow(clippy::redundant_feature_names, reason = "rust-for-linux")]
#![allow(clippy::redundant_pub_crate, reason = "rust-for-linux")]
#![allow(clippy::wildcard_dependencies, reason = "rust-for-linux")]

//! Tenstorrent device driver module.

use ::kernel::prelude::*;

mod tt;

module! {
    type: TtDriverModule,
    name: "tt_core",
    authors: ["Darin Morrison"],
    description: "tenstorrent driver (rust)",
    license: "GPL v2",
}

/// The initialized driver module state.
#[allow(clippy::empty_structs_with_brackets)]
struct TtDriverModule {}

impl ::kernel::Module for TtDriverModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let version = tt::version::version()?;
        let version = version.to_str()?;
        pr_info!("(init): {version}\n");
        Ok(Self {})
    }
}
