# snr8503x-hal/examples

This project contains in the src/bin examples of how to use the hal
Each binary can be build with 

    > cargo build --release --bin <example>


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
