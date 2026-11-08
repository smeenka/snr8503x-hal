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

    
