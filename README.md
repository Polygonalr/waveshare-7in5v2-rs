# waveshare-rpi

*This module is currently under **heavy development** and is written for code porting practice. You should be using [epd-waveshare](https://lib.rs/crates/epd-waveshare) instead, which supports more embedded host devices and Waveshare displays.*

`waveshare-rpi` is a (unpublished) crate for Raspberry Pis to use Waveshare e-paper displays. Currently, it only supports the following displays, though more displays can be easily added.

* 7.5inch E-Paper Display V2
* *More to come?*

## Example library usage

```rust
use waveshare_rpi::{epd::epd7in5_v2::EPD_CONFIG, Epd};
use waveshare_rpi::util::{ColorMode, image_to_epd};

// Initialize the interface to interact with the epd7in5_v2 display
let mut my_epd = Epd::new(EPD_CONFIG);

// Resizes and converts image.png into the format compatible with the epd7in5_v2 display
let img_data = image_to_epd("image.png", ColorMode::BlackWhite, EPD_CONFIG.width, EPD_CONFIG.height).unwrap();

// Transfer the image data to the display for displaying
my_epd.display(&img_data);
```

## Compilation for testing

For testing purposes, this project has a `main.rs` file which can be modified and compiled for testing on a Raspberry Pi.

### Cross-compiling from Ubuntu 22.04

Directly compiling for Raspberry Pi OS `bullseye` (which is the latest version of Raspberry Pi OS as of writing this) is currently not supported due to the version of the `libc` linker on the Ubuntu machine being too new - `bullseye` by default does not have the newer `libc` versions. Therefore, it is a requirement to compile for the target `aarch64-unknown-linux-musl` to use static linking. However, using `gcc` may still give some problems, so `clang` is used instead.

* Install the required tools with the following command. 

```
sudo apt-get install musl-tools clang llvm -y
```

* To tell `cargo` to use `clang` as the linker, export the following environment values.

```
export CC_aarch64_unknown_linux_musl=clang
export AR_aarch64_unknown_linux_musl=llvm-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
```

* Finally, compile the project with the following command.

```
cargo build --release --target aarch64-unknown-linux-musl
```

The compiled program can be found at `target/aarch64-unknown-linux-musl/release` and can be transferred over to a Raspberry Pi to be executed.

Usage of the program:

```
./waveshare-rpi --help
Program to update a Waveshare 7.5" e-ink display

Usage: waveshare-rpi [OPTIONS]

Options:
  -i, --image <IMAGE>  Path of the image to display
  -t, --text <TEXT>    Text to display
  -c, --clear          Clear the display
  -h, --help           Print help
  -V, --version        Print version
```
