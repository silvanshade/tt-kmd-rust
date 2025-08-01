# Tenstorrent kernel-mode driver (rust)

A Linux kernel-mode driver in Rust for Tenstorrent devices.

## Status

This driver is unofficial and experimental and not ready for production use.

### Supported Functionality

The following functionality has been implemented:

- [x] successfully compiles to a kernel module
- [x] module loads and recognizes PCI vendor id, device id, and device name

### Roadmap

The following functionality is planned but not yet implemented:

- [ ] initialize device firmware
- [ ] support `tt-smi` features

## Contributing

Contributions and feedback are very much welcome.

Feel free to open an issue or message me on the [Tenstorrent Discord](https://discord.gg/tvhGzHQwaj).

## Building

Building and loading this module requires the following:

1. Kernel compiled with [Rust support](https://cateee.net/lkddb/web-lkddb/RUST.html) and recent Rust-for-Linux patches (see below).
2. Kernel header installation from (1).
3. Kernel source tree from (1) if needing rust-analyzer support. (optional)

This module is built out-of-tree and requires some features from the [Rust-for-Linux](https://rust-for-linux.com/) project which are not yet included in the mainline kernel.

The [patches](./patches) directory contains the collection of specific patches needed to apply on top of a given kernel branch in order to be able to compile a kernel compatible with this module.

Details regarding the origin of these patches is provided in the README files.
