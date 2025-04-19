# Tabata rust

This is a learning project to teach myself Rust, specifically for embedded devices.

## Idea

I had some parts lying around and wanted to get familiar with some Rust concepts.
TODO

## Ressources
### Hardware
- [feather rp2040 from Adafruit](https://learn.adafruit.com/adafruit-feather-rp2040-pico/overview)
- [ST7789 display from Adafruit](https://learn.adafruit.com/adafruit-1-69-280x240-round-rectangle-color-ips-tft-display)
- standard rotary encoder

### Software
- [Embassy RTOS for rp20xx](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html)
- [rp2040_hal crate](https://docs.rs/rp2040-hal/latest/rp2040_hal/)
- [embedded_graphics crate](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
- [mipdsi crate for interfaces with ST7789 display](https://docs.rs/mipidsi/latest/mipidsi/index.html)

## Architecture

The embedded-graphics crate defines the `DrawTarget` trait which is implemented by their simulator (from the embedded-graphics-simulator crate) and from the display interface from the mipidsi crate.
TODO

## Building

Haven't fully figured this one out yet, for now run:

```sh
cargo build -p tabata-core
cargo build -p tabata-simulator
cargo build -p tabata-rp2040 --target thumbv6m-none-eabi
```