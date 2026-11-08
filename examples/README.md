# snr8503-rs

--------

## Build

```
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
cargo build --release
cargo objcopy --release -- -O binary target.bin
```
cargo objcopy --release -- -O ihex target.hex
