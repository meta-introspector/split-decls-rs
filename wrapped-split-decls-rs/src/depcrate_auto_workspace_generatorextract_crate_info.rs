// Generated macro for extract_crate_info (function)
macro_rules! Depcrate_auto_workspace_generatorextract_crate_info {
() => {
// Module: crate::auto_workspace_generator
// Provides: {"extract_crate_info"}
// Dependencies: {}
fn extract_crate_info (cargo_path : & Path) -> Result < Option < CrateInfo > > { let content = fs :: read_to_string (cargo_path) ? ; let toml : Value = toml :: from_str (& content) ? ; if let Some (package) = toml . get ("package") { if let Some (name) = package . get ("name") . and_then (| n | n . as_str ()) { return Ok (Some (CrateInfo { name : name . to_string () , })) ; } } Ok (None) }
};
}
