// Generated macro for RustConfig (struct)
macro_rules! Depcrate_rustRustConfig {
() => {
// Module: crate::rust
// Provides: {"RustConfig"}
// Dependencies: {}
# [doc = " Rust-specific configuration of component files"] # [derive (Default , Deserialize)] # [serde (deny_unknown_fields)] struct RustConfig { # [doc = " Space-separated list or array of compiler flags to pass."] # [serde (default)] rustflags : StringList , # [doc = " List of path to rust files to build as external crates and link to the"] # [doc = " main crate."] # [serde (default)] externs : Vec < String > , }
};
}
