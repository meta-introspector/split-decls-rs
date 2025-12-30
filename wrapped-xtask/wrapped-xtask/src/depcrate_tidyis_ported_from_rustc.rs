// Generated macro for is_ported_from_rustc (function)
macro_rules! Depcrate_tidyis_ported_from_rustc {
() => {
// Module: crate::tidy
// Provides: {"is_ported_from_rustc"}
// Dependencies: {}
fn is_ported_from_rustc (p : & Path , dirs_to_exclude : & [& str]) -> bool { let p = p . strip_prefix (project_root ()) . unwrap () ; dirs_to_exclude . iter () . any (| exclude | p . starts_with (exclude)) }
};
}
