// Generated macro for load_config (function)
macro_rules! Depcrate_configload_config {
() => {
// Module: crate::config
// Provides: {"load_config"}
// Dependencies: {}
# [doc = " Loads a config by checking the client-supplied options and if appropriate, the"] # [doc = " file system (including searching the file system for overrides)."] pub fn load_config < O : CliOptions > (file_path : Option < & Path > , options : Option < O > ,) -> Result < (Config , Option < PathBuf >) , Error > { let (over_ride , edition , style_edition , version) = match options { Some (ref opts) => (config_path (opts) ? , opts . edition () , opts . style_edition () , opts . version () ,) , None => (None , None , None , None) , } ; let result = if let Some (over_ride) = over_ride { Config :: from_toml_path (over_ride . as_ref () , edition , style_edition , version) . map (| p | (p , Some (over_ride . to_owned ()))) } else if let Some (file_path) = file_path { Config :: from_resolved_toml_path (file_path , edition , style_edition , version) } else { Ok ((Config :: default_for_possible_style_edition (style_edition , edition , version) , None ,)) } ; result . map (| (mut c , p) | { if let Some (options) = options { options . apply_to (& mut c) ; } (c , p) }) }
};
}
