// Generated macro for resolve_dir (function)
macro_rules! Depcrate_dir_opsresolve_dir {
() => {
// Module: crate::dir::ops
// Provides: {"resolve_dir"}
// Dependencies: {}
pub fn resolve_dir (path : impl AsRef < std :: path :: Path > ,) -> Result < std :: path :: PathBuf , std :: io :: Error > { let path = path . as_ref () ; let meta = std :: fs :: symlink_metadata (path) ? ; if meta . is_dir () { canonicalize (path) } else if meta . is_file () { let target = std :: fs :: read_to_string (path) ? ; let target_path = path . parent () . unwrap () . join (target) ; resolve_dir (target_path) } else { canonicalize (path) } }
};
}
