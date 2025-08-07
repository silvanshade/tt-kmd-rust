# patches/kernel/6.16.0

Patches for the `linux-6.16.0` kernel.

## Patch sets

The following patch sets are applied in order:

1. [[PATCH v6 0/9] LKMM generic atomics in Rust](https://lore.kernel.org/rust-for-linux/20250710060052.11955-1-boqun.feng@gmail.com/)
2. [[PATCH v17 0/7] rust: extend `module!` macro with integer parameter support](https://lore.kernel.org/rust-for-linux/20250711-module-params-v3-v17-0-cf9b10d4923d@kernel.org/)
3. [[PATCH v2 0/3] rust: xarray: add `insert` and `reserve`](https://lore.kernel.org/rust-for-linux/20250713-xarray-insert-reserve-v2-0-b939645808a2@gmail.com/)
4. Patches developed for this module not yet submitted to LKML.

## Applying the patches directly

The patches can be applied in order with `git`:

```shell
cd ~/Development/linux-6.16.0
git apply -3 ../tt-kmd-rust/patches/kernel/6.16.0/*
```

## Fetching and applying from the origin

Alternatively, the patch sets can be fetched from their origin threads on [lore.kernel.org](https://lore.kernel.org) with `b4` then applied with `git`:

```shell
b4 am -cgk3 <message-id>    # find from the 'Message-ID: ...' field
git am -3 <fetched.mbx>     # replace with corresponding .mbx file
```
