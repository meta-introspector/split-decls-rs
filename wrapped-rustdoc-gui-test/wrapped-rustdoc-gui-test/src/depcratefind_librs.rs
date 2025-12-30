// Generated macro for find_librs (function)
macro_rules! Depcratefind_librs {
() => {
// Module: crate
// Provides: {"find_librs"}
// Dependencies: {}
fn find_librs < P : AsRef < Path > > (path : P) -> Option < PathBuf > { for entry in walkdir :: WalkDir :: new (path) { let entry = entry . ok () ? ; if entry . file_type () . is_file () && entry . file_name () == "lib.rs" { return Some (entry . path () . to_path_buf ()) ; } } None }
};
}
