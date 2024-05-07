# SimConnect Bindings for Rust

## Why this crate
why is this different from many of the other crates? 
1. No Bindgen, this projects only dependency is cargo
2. Static linking feature - no more SimConnect.dll needed

## Using
Add this to your `Cargo.toml`
```toml
[dependencies]
simconnect = {git="https://github.com/Tacotakedown/simconnect-rs" ,features=["static-link"]}
```
if you prefer dynamic linking omit the feature and ensure you have `SimConnect.dll` next to your exe

## Building
*The latest SimConnect binaries are included within this repository.*

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

