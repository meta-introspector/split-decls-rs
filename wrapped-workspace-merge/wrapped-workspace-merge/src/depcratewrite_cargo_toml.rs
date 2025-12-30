// Generated macro for write_cargo_toml (function)
macro_rules! Depcratewrite_cargo_toml {
() => {
// Module: crate
// Provides: {"write_cargo_toml"}
// Dependencies: {}
fn write_cargo_toml (path : & Path , toml_data : & CargoToml) -> Result < () > { let toml_string = toml :: to_string_pretty (toml_data) . context (format ! ("Failed to serialize Cargo.toml data for {:?}" , path)) ? ; fs :: write (path , toml_string) . context (format ! ("Failed to write Cargo.toml to {:?}" , path)) ? ; Ok (()) }
};
}
