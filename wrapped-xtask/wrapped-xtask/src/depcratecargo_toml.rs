// Generated macro for cargo_toml (function)
macro_rules! Depcratecargo_toml {
() => {
// Module: crate
// Provides: {"cargo_toml"}
// Dependencies: {}
pub fn cargo_toml () -> Result < CargoToml > { let manifest_dir = env :: var ("CARGO_MANIFEST_DIR") ? ; let path = PathBuf :: from (manifest_dir) . join ("Cargo.toml") ; let contents = std :: fs :: read_to_string (& path) ? ; Ok (CargoToml { path , contents }) }
};
}
