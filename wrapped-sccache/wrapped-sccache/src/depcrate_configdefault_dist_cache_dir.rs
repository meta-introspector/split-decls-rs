// Generated macro for default_dist_cache_dir (function)
macro_rules! Depcrate_configdefault_dist_cache_dir {
() => {
// Module: crate::config
// Provides: {"default_dist_cache_dir"}
// Dependencies: {}
pub fn default_dist_cache_dir () -> PathBuf { ProjectDirs :: from ("" , ORGANIZATION , DIST_APP_NAME) . expect ("Unable to retrieve dist cache directory") . cache_dir () . to_owned () }
};
}
