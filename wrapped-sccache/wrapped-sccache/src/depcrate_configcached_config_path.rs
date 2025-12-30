// Generated macro for CACHED_CONFIG_PATH (static)
macro_rules! Depcrate_configCACHED_CONFIG_PATH {
() => {
// Module: crate::config
// Provides: {"CACHED_CONFIG_PATH"}
// Dependencies: {}
static CACHED_CONFIG_PATH : LazyLock < PathBuf > = LazyLock :: new (CachedConfig :: file_config_path) ;
};
}
