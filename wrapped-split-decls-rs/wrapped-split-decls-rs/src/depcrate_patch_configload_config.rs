// Generated macro for load_config (function)
macro_rules! Depcrate_patch_configload_config {
() => {
// Module: crate::patch_config
// Provides: {"load_config"}
// Dependencies: {}
pub fn load_config () -> Result < SplitDeclsConfig > { let config_path = "split-decls-rs.toml" ; let config_content = std :: fs :: read_to_string (config_path) ? ; let config : SplitDeclsConfig = toml :: from_str (& config_content) ? ; Ok (config) }
};
}
