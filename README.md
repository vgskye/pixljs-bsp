# Board Support Package for the {manufacturer} {board}

This crate is a Board Support Package (BSP). It wraps the HAL crate ({hal
crate name}) for the on-board {CPU}, and provides high level wrappers for the
onboard features:

* 29 on-board LEDs
* Bosch XX123 I²C Temperature Sensor
* 5 on-board buttons
* Add your actual on-board components here

This BSP assumes you are not using a bootloader / using the XXXX bootloader /
running in non-secure mode (delete as applicable).

## Usage

You will require the `thumbv7em-none-eabihf` target installed. To build one of these examples:

```console
$ rustup target add thumbv7em-none-eabihf
$ git clone https://github.com/nrf-rs/nrf-bsp-template.git
$ cd nrf-bsp-template
$ cargo build --target=thumbv7em-none-eabihf --example blinky
```

To use in your own application, add as a dependency and call the
`Board::init()` function.

## Documentation

The docs for this crate can be found at https://docs.rs/nrf-bsp-yourcrate. The
manufacturer's documentation is available from
https://example.com/path/to/manual.pdf.

## Changelog

See [CHANGELOG.md].

## Minimum Supported Rust Version

This crate is guaranteed to build on stable Rust 1.41 and higher.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
