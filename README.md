# snr8503x-hal

A hal layer for the chip snr8505x as found on cheap chinese motordriver boards

Hal copied from https://github.com/qnnnnez/snr8503x-rs

The stm32g0xx-hal is taken as example.

--------

## Build

### Starting:

    > rustup target add thumbv6m-none-eabi
    > rustup component add llvm-tools-preview
    > cargo install cargo-binutils

### Building blinky (as example)

    > cd examples
    > cargo build --release --bin blinky

### Flashing 

At the moment there is no support for probe-rs.
Building and flashing in one go is supported with a simple make file, which supports the different OSes.

Install and build openocd:

    > cd /your/data/location
    > git clone https://github.com/andrewthecat/openocd-snr8503
    > cd openocd-snr8503 
    > ./bootstrap
    > ./configure
    > make

Install the make tool for your platform:

Edit the Makefile in examples/Makefile, and change BINARY and OPENOCD_DIR settings.

For a pico-probe the setting for OPENOCD_INTERFACE is fine. For different probe one has to change this setting.

Flashing:

    powercycle the board
    >make flash
    
