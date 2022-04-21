# vedirect-prom
A Vedirect-serial to Prometheus bridge

## Build

Clone vedirect-prom and vedirect-rs from my repositories:

    git clone https://github.com/rp-/vedirect-prom.git
    git clone https://github.com/rp-/vedirect-rs.git

Switch in the `vedirect-rs` repo to the `mppt` branch

    cd vedirect-rs
    git switch mppt

Then build it like any other rust project:

    cd vedirect-prom
    cargo build

If you want to cross-compile for raspberry you can use
`make` to create images and build in them, e.g.:

    make image-raspberry-armv7
    make build-raspberry-armv7-release
