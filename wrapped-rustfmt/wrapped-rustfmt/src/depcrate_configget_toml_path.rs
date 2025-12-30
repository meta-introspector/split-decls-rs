// Generated macro for get_toml_path (function)
macro_rules! Depcrate_configget_toml_path {
() => {
// Module: crate::config
// Provides: {"get_toml_path"}
// Dependencies: {}
fn get_toml_path (dir : & Path) -> Result < Option < PathBuf > , Error > { const CONFIG_FILE_NAMES : [& str ; 2] = [".rustfmt.toml" , "rustfmt.toml"] ; for config_file_name in & CONFIG_FILE_NAMES { let config_file = dir . join (config_file_name) ; match fs :: metadata (& config_file) { Ok (ref md) if md . is_file () => return Ok (Some (config_file . canonicalize () ?)) , Err (e) => { if e . kind () != ErrorKind :: NotFound { let ctx = format ! ("Failed to get metadata for config file {:?}" , & config_file) ; let err = anyhow :: Error :: new (e) . context (ctx) ; return Err (Error :: new (ErrorKind :: Other , err)) ; } } _ => { } } } Ok (None) }
};
}
