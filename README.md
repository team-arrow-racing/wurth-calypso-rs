[![crates.io](https://img.shields.io/crates/v/wurth-calypso.svg)](https://crates.io/crates/wurth-calypso)
[![crates.io](https://img.shields.io/crates/d/wurth-calypso.svg)](https://crates.io/crates/wurth-calypso)

# Wurth Calypso Embedded Driver

This crate imeplements a driver for the Würth Elektronik Calypso Wi-Fi radio module.

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
wurth-calypso = "0.1.0"
```

You can also enable the optional `defmt` feature.

## Running the examples

You can run one of the examples like this, remembering to substitute your own serial device instead of `/dev/ttyUSB0`.

```shell
cargo run --example test -- /dev/ttyUSB0
```

To see what's going on under the hood, you can turn on trace logging by setting the environment variable `RUST_LOG=trace`.

## References

- [Calypso Reference Manual Version 2.4](https://www.we-online.com/components/products/manual/2610011025000_Calypso%20261001102500x%20Manual_rev2.4.pdf)

## License

Licensed under [Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
