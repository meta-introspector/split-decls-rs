// Generated macro for find_all_cargo_tomls (function)
macro_rules! Depcrate_find_all_cargo_tomlsfind_all_cargo_tomls {
() => {
// Module: crate::find_all_cargo_tomls
// Provides: {"find_all_cargo_tomls"}
// Dependencies: {}
pub fn find_all_cargo_tomls (dir : & Path , verbose : bool) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if verbose { println ! ("DEBUG: find_all_cargo_tomls: Scanning directory: {}" , dir . display ()) ; } for entry in WalkDir :: new (dir) . into_iter () . filter_map (| e | e . ok ()) { let path = entry . path () ; if path . is_file () && path . file_name () . map_or (false , | name | name == "Cargo.toml") { if verbose { println ! ("DEBUG: Found Cargo.toml: {}" , path . display ()) ; } cargo_tomls . push (path . to_path_buf ()) ; } } if verbose { println ! ("DEBUG: Finished scanning. Found {} Cargo.toml files." , cargo_tomls . len ()) ; } Ok (cargo_tomls) }
};
}
