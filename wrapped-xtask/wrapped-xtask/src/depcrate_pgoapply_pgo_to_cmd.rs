// Generated macro for apply_pgo_to_cmd (function)
macro_rules! Depcrate_pgoapply_pgo_to_cmd {
() => {
// Module: crate::pgo
// Provides: {"apply_pgo_to_cmd"}
// Dependencies: {}
pub (crate) fn apply_pgo_to_cmd < 'a > (cmd : Cmd < 'a > , profile_path : & Path) -> Cmd < 'a > { cmd . env ("RUSTFLAGS" , format ! ("-Cprofile-use={}" , profile_path . to_str () . unwrap ())) }
};
}
