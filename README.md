## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## The setup

[book]: https://rust-embedded.github.io/book


In this example we'll be using the STM32F412DISCOVERY. This board contains an
STM32F412zg microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 1024 KiB of Flash located at address 0x0800_0000.

- 256 KiB of RAM located at address 0x2000_0000.

