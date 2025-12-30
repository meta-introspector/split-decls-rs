// Generated macro for lookup_in_path (function)
macro_rules! Depcratelookup_in_path {
() => {
// Module: crate
// Provides: {"lookup_in_path"}
// Dependencies: {}
fn lookup_in_path (exec : & str) -> Option < Utf8PathBuf > { let paths = env :: var_os ("PATH") . unwrap_or_default () ; env :: split_paths (& paths) . map (| path | path . join (exec)) . map (Utf8PathBuf :: try_from) . filter_map (Result :: ok) . find_map (probe_for_binary) }
};
}
