// Generated macro for maybe_add_cargo_toml (function)
macro_rules! Depcrate_compiler_rustmaybe_add_cargo_toml {
() => {
// Module: crate::compiler::rust
// Provides: {"maybe_add_cargo_toml"}
// Dependencies: {}
# [cfg (feature = "dist-client")] fn maybe_add_cargo_toml (input_path : & Path , verify : bool) -> Option < PathBuf > { let lib_rs = PathBuf :: new () . join ("src") . join ("lib.rs") ; if input_path . ends_with (lib_rs) { let cargo_toml_path = input_path . parent () . expect ("No parent") . parent () . expect ("No parent") . join ("Cargo.toml") ; if cargo_toml_path . is_file () || ! verify { Some (cargo_toml_path) } else { None } } else { None } }
};
}
