// Generated macro for lookup_as_env_var (function)
macro_rules! Depcratelookup_as_env_var {
() => {
// Module: crate
// Provides: {"lookup_as_env_var"}
// Dependencies: {}
# [doc = " Looks up the binary as its SCREAMING upper case in the env variables."] fn lookup_as_env_var (executable_name : & str) -> Option < Utf8PathBuf > { env :: var_os (executable_name . to_ascii_uppercase ()) . map (PathBuf :: from) . map (Utf8PathBuf :: try_from) . and_then (Result :: ok) }
};
}
