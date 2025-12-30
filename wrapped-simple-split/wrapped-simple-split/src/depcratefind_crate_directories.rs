// Generated macro for find_crate_directories (function)
macro_rules! Depcratefind_crate_directories {
() => {
// Module: crate
// Provides: {"find_crate_directories"}
// Dependencies: {}
fn find_crate_directories () -> Result < Vec < PathBuf > > { let mut crate_dirs = Vec :: new () ; let essential_crates = [".." , "../crates/split-decls-types" , "../crates/cargo-toml-generator-types" , "../crates/cargo-toml-generator-macros" , "../crates/cargo-toml-parts" , "../crates/pagerank_rs" , "../crates/introspector_decl2_macros" , "../crates/introspector_decl_common" , "../crates/introspector_decl_core" , "../crates/introspector_macro_helpers" ,] ; for crate_path in & essential_crates { let path = Path :: new (crate_path) ; if path . join ("Cargo.toml") . exists () && path . join ("src") . exists () { crate_dirs . push (path . to_path_buf ()) ; } } Ok (crate_dirs) }
};
}
