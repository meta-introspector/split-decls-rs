// Generated macro for find_cargo_tomls (function)
macro_rules! Depcrate_auto_workspace_generatorfind_cargo_tomls {
() => {
// Module: crate::auto_workspace_generator
// Provides: {"find_cargo_tomls"}
// Dependencies: {}
fn find_cargo_tomls (dir : & Path) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . file_name () == Some ("Cargo.toml" . as_ref ()) { cargo_tomls . push (path) ; } else if path . is_dir () && ! should_skip_dir (& path) { cargo_tomls . extend (find_cargo_tomls (& path) ?) ; } } } Ok (cargo_tomls) }
};
}
