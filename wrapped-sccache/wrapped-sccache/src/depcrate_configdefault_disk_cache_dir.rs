// Generated macro for default_disk_cache_dir (function)
macro_rules! Depcrate_configdefault_disk_cache_dir {
() => {
// Module: crate::config
// Provides: {"default_disk_cache_dir"}
// Dependencies: {}
pub fn default_disk_cache_dir () -> PathBuf { ProjectDirs :: from ("" , ORGANIZATION , APP_NAME) . expect ("Unable to retrieve disk cache directory") . cache_dir () . to_owned () }
};
}
