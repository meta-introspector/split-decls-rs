// Generated macro for RustOpts (struct)
macro_rules! Depcrate_rustRustOpts {
() => {
// Module: crate::rust
// Provides: {"RustOpts"}
// Dependencies: {}
# [derive (Default , Debug , Clone , Parser)] pub struct RustOpts { # [doc = " A custom `path` dependency to use for `wit-bindgen`."] # [clap (long , conflicts_with = "rust_wit_bindgen_version" , value_name = "PATH")] rust_wit_bindgen_path : Option < PathBuf > , # [doc = " A custom version to use for the `wit-bindgen` dependency."] # [clap (long , conflicts_with = "rust_wit_bindgen_path" , value_name = "X.Y.Z")] rust_wit_bindgen_version : Option < String > , # [doc = " Name of the Rust target to compile for."] # [clap (long , default_value = "wasm32-wasip2" , value_name = "TARGET")] rust_target : String , }
};
}
