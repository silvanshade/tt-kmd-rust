# tt-kmd-rust build guide

## Prerequisites

Building and loading `tt-kmd-rust` requires:

1. Kernel compiled with [Rust support](https://cateee.net/lkddb/web-lkddb/RUST.html) and recent Rust-for-Linux patches (see below).
2. Kernel header installation from (1).
3. Kernel source tree from (1) if needing rust-analyzer support. (optional)

This module is built out-of-tree and uses some features from the [Rust-for-Linux](https://rust-for-linux.com/) project which are not yet included in the mainline kernel.

The [patches](../../patches) directory contains the specific patches needed to apply to a given kernel branch in order to compile a compatible kernel.
