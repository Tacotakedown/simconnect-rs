# SimConnect Bindings for Rust

## Updating the bindings
why is this different from many of the other crates? 
1. No Bindgen, this projects only dependency is cargo
2. Static link (that may or may not work), this should remove the need for the SimConnect.dll to be present next to your compiled exe

## Using
Add this to your `Cargo.toml`
```toml
[dependencies]
simconnect = {git="https://github.com/Tacotakedown/simconnect-rs" features=["static-link"]}
```

## Building
*The SimConnect binaries are included within this repository, but they may not be up-to-date.*

1. run `cargo build`
2. Add `use simconnect` at the top of your file

## Example
Read float position data

```
cargo run --example aircraft_updates
```

Requests tagged data with thresholds from SimConnect and reads floats/strings
```
cargo run --example aircraft_updates_on_change
```

*You must have SimConnect.dll in the same directory as the compiled exe for it to run (e.g. in )*

### Remarks
I have not tested every single function from the api. If you find an error, feel free to make an issue or a pull request.
