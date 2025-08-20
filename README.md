# Rust UEFI Runtime Driver sample

## build

```
cargo build --release
```

## run

```
cargo run --release
```

## debug

```
cargo run
```

In another terminal:

```
uv sync  // only required the first time
./dbg.sh
```

Or you can debug by VSCode Native Debug extension.

## important points

### `uefi-runtime-driver-debug.json`

This custom target generates EFI binary with DWARF symbols.

```jsonc
{
  "debuginfo-kind": "dwarf" /* here */,
  /* ...(snip)... */
  "pre-link-args": {
    "msvc": [
      "/NOLOGO",
      "/entry:efi_main",
      "/subsystem:efi_runtime_driver" /* here */,
      "/debug:dwarf" /* here */
    ],
    "msvc-lld": [
      "/NOLOGO",
      "/entry:efi_main",
      "/subsystem:efi_runtime_driver" /* here */,
      "/debug:dwarf" /* here */
    ]
  },
  /* ...(snip)... */
  "split-debuginfo": "unpacked" /* here */,
  /* ...(snip)... */
  "supported-split-debuginfo": ["unpacked"] /* here */
  /* ...(snip)... */
}
```

### `load-symbols.py`

Use this script while debugging with GDB.

The scripts is from [rust-uefi-runtime-driver](https://github.com/x1tan/rust-uefi-runtime-driver) .
Thanks [@x1tan](https://github.com/x1tan) .

## Reference

- [rust-uefi-runtime-driver](https://github.com/x1tan/rust-uefi-runtime-driver)
- [How to debug UEFI App](https://github.com/rust-osdev/uefi-rs/issues/289#issuecomment-2705938540)
- [VSCode Native Debug issue](https://github.com/WebFreak001/code-debug/issues/454#issuecomment-2684620194)
