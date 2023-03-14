# vedirect-prom
A Vedirect-serial to Prometheus bridge

## Build

Build it like any other rust project:

    cd vedirect-prom
    cargo build

If you want to cross-compile for raspberry you can use
`make` to create images and build in them, e.g.:

    make image-raspberry-armv7
    make build-raspberry-armv7-release
