// Generated macro for display_relpath (function)
macro_rules! Depcrate_dir_opsdisplay_relpath {
() => {
// Module: crate::dir::ops
// Provides: {"display_relpath"}
// Dependencies: {}
pub (crate) fn display_relpath (path : impl AsRef < std :: path :: Path >) -> String { let path = path . as_ref () ; let relpath = if let Ok (cwd) = std :: env :: current_dir () { match path . strip_prefix (cwd) { Ok (path) => path , Err (_) => path , } } else { path } ; relpath . display () . to_string () }
};
}
