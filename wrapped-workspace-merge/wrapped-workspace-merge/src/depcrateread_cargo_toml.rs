// Generated macro for read_cargo_toml (function)
macro_rules! Depcrateread_cargo_toml {
() => {
// Module: crate
// Provides: {"read_cargo_toml"}
// Dependencies: {}
fn read_cargo_toml (path : & Path) -> Result < CargoToml > { let content = fs :: read_to_string (path) . context (format ! ("Failed to read Cargo.toml from {:?}" , path)) ? ; let toml : CargoToml = toml :: from_str (& content) . context (format ! ("Failed to parse Cargo.toml from {:?}" , path)) ? ; Ok (toml) }
};
}
