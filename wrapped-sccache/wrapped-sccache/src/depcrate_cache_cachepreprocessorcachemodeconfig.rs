// Generated macro for PreprocessorCacheModeConfig (struct)
macro_rules! Depcrate_cache_cachePreprocessorCacheModeConfig {
() => {
// Module: crate::cache::cache
// Provides: {"PreprocessorCacheModeConfig"}
// Dependencies: {}
# [doc = " Configuration switches for preprocessor cache mode."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] # [serde (default)] pub struct PreprocessorCacheModeConfig { # [doc = " Whether to use preprocessor cache mode entirely"] pub use_preprocessor_cache_mode : bool , # [doc = " If false (default), only compare header files by hashing their contents."] # [doc = " If true, will use size + ctime + mtime to check whether a file has changed."] # [doc = " See other flags below for more control over this behavior."] pub file_stat_matches : bool , # [doc = " If true (default), uses the ctime (file status change on UNIX,"] # [doc = " creation time on Windows) to check that a file has/hasn't changed."] # [doc = " Can be useful to disable when backdating modification times"] # [doc = " in a controlled manner."] pub use_ctime_for_stat : bool , # [doc = " If true, ignore `__DATE__`, `__TIME__` and `__TIMESTAMP__` being present"] # [doc = " in the source code. Will speed up preprocessor cache mode,"] # [doc = " but can result in false positives."] pub ignore_time_macros : bool , # [doc = " If true, preprocessor cache mode will not cache system headers, only"] # [doc = " add them to the hash."] pub skip_system_headers : bool , # [doc = " If true (default), will add the current working directory in the hash to"] # [doc = " distinguish two compilations from different directories."] pub hash_working_directory : bool , }
};
}
