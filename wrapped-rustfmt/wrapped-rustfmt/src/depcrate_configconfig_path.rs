// Generated macro for config_path (function)
macro_rules! Depcrate_configconfig_path {
() => {
// Module: crate::config
// Provides: {"config_path"}
// Dependencies: {}
fn config_path (options : & dyn CliOptions) -> Result < Option < PathBuf > , Error > { let config_path_not_found = | path : & str | -> Result < Option < PathBuf > , Error > { Err (Error :: new (ErrorKind :: NotFound , format ! ("Error: unable to find a config file for the given path: `{}`" , path) ,)) } ; match options . config_path () { Some (path) if ! path . exists () => config_path_not_found (path . to_str () . unwrap ()) , Some (path) if path . is_dir () => { let config_file_path = get_toml_path (path) ? ; if config_file_path . is_some () { Ok (config_file_path) } else { config_path_not_found (path . to_str () . unwrap ()) } } Some (path) => Ok (Some (path . canonicalize () ? ,)) , None => Ok (None) , } }
};
}
