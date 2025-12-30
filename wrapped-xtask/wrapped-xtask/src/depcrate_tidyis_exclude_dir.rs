// Generated macro for is_exclude_dir (function)
macro_rules! Depcrate_tidyis_exclude_dir {
() => {
// Module: crate::tidy
// Provides: {"is_exclude_dir"}
// Dependencies: {}
fn is_exclude_dir (p : & Path , dirs_to_exclude : & [& str]) -> bool { p . strip_prefix (project_root ()) . unwrap () . components () . rev () . skip (1) . filter_map (| it | it . as_os_str () . to_str ()) . any (| it | dirs_to_exclude . contains (& it)) }
};
}
