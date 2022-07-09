rustc ./src/main.rs --crate-type lib
rustdoc --extern doc="libmain.rlib" ./src/main.rs
cargo doc --open