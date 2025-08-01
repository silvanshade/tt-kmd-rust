<div align="center">
  <h1><code>tt-kmd-rust</code></h1>
  <p>
    <strong>A <a href="https://rust-for-linux.com/">Rust-for-Linux</a> kernel-mode driver for <a href="https://tenstorrent.com">Tenstorrent</a> devices</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://github.com/silvanshade/tt-kmd-rust/actions"><img
        src="https://github.com/silvanshade/tt-kmd-rust/workflows/ci/badge.svg"
        /></a>
  </p>
</div>

## Status

This driver is unofficial and experimental and not production-ready.

### Supported Functionality

- [x] successfully compiles to a kernel module
- [x] module loads and recognizes PCI vendor id, device id, and device name
- [x] creates misc device(s) under `/dev/tenstorrent/n`

### Planned Functionality

- [ ] initialize device firmware
- [ ] support `tt-smi` features

### Roadmap

The long-term goal of the project is to reach and maintain feature parity with the official [Tenstorrent Kernel Module](https://github.com/tenstorrent/tt-kmd).

## Contributing

Contributions and feedback are very much welcome.

Feel free to submit a PR or open an issue or message me on the [Tenstorrent Discord](https://discord.gg/tvhGzHQwaj).

## Documentation

See [docs](./docs) directory for documentation about the `tt-kmd-rust`.

### Usage

- [building](./docs/usage/building.md)
- [installing](./docs/usage/installing.md)

### Development

- [architecture](./docs/development/architecture.md)
