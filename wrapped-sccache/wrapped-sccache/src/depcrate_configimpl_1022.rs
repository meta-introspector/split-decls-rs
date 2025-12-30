// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_configimpl_1022 {
() => {
// Module: crate::config
// Provides: {"impl_1022"}
// Dependencies: {}
impl Config { pub fn load () -> Result < Self > { let env_conf = config_from_env () ? ; let file_conf_path = config_file ("SCCACHE_CONF" , "config") ; let file_conf = try_read_config_file (& file_conf_path) . context ("Failed to load config file") ? . unwrap_or_default () ; Ok (Self :: from_env_and_file_configs (env_conf , file_conf)) } fn from_env_and_file_configs (env_conf : EnvConfig , file_conf : FileConfig) -> Self { let mut conf_caches : CacheConfigs = Default :: default () ; let FileConfig { cache , dist , server_startup_timeout_ms , } = file_conf ; conf_caches . merge (cache) ; let server_startup_timeout = server_startup_timeout_ms . map (std :: time :: Duration :: from_millis) ; let EnvConfig { cache } = env_conf ; conf_caches . merge (cache) ; let (caches , fallback_cache) = conf_caches . into_fallback () ; Self { cache : caches , fallback_cache , dist , server_startup_timeout , } } }
};
}
