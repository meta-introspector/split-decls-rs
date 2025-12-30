// Generated macro for key_prefix_from_env_var (function)
macro_rules! Depcrate_configkey_prefix_from_env_var {
() => {
// Module: crate::config
// Provides: {"key_prefix_from_env_var"}
// Dependencies: {}
fn key_prefix_from_env_var (env_var_name : & str) -> String { env :: var (env_var_name) . ok () . as_ref () . map (| s | s . trim_end_matches ('/')) . filter (| s | ! s . is_empty ()) . unwrap_or_default () . to_owned () }
};
}
