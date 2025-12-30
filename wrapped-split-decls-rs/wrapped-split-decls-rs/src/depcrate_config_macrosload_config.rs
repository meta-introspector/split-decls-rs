// Generated macro for load_config (macro)
macro_rules! Depcrate_config_macrosload_config {
() => {
// Module: crate::config_macros
// Provides: {"load_config"}
// Dependencies: {}
# [macro_export] macro_rules ! load_config { ($ path : expr) => { let content = std :: fs :: read_to_string ($ path) . unwrap () ; let config : split_decls_types :: SplitDeclsConfig = toml :: from_str (& content) . unwrap () ; * crate :: config_macros :: GLOBAL_CONFIG . lock () . unwrap () = config ; } ; }
};
}
