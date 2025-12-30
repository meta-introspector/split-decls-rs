// Generated macro for override_temp_dir (function)
macro_rules! Depcrate_envoverride_temp_dir {
() => {
// Module: crate::env
// Provides: {"override_temp_dir"}
// Dependencies: {}
# [doc = " Override the default temporary directory (defaults to [`std::env::temp_dir`]). This function"] # [doc = " changes the _global_ default temporary directory for the entire program and should not be called"] # [doc = " except in exceptional cases where it's not configured correctly by the platform. Applications"] # [doc = " should first check if the path returned by [`env::temp_dir`] is acceptable."] # [doc = ""] # [doc = " If you're writing a library and want to control where your temporary files are placed, you"] # [doc = " should instead use the `_in` variants of the various temporary file/directory constructors"] # [doc = " ([`tempdir_in`], [`tempfile_in`], the so-named functions on [`Builder`], etc.)."] # [doc = ""] # [doc = " Only the first call to this function will succeed. All further calls will fail with `Err(path)`"] # [doc = " where `path` is previously set default temporary directory override."] # [doc = ""] # [doc = " **NOTE:** This function does not check if the specified directory exists and/or is writable."] pub fn override_temp_dir (path : & Path) -> Result < () , PathBuf > { let mut we_set = false ; let val = DEFAULT_TEMPDIR . get_or_init (| | { we_set = true ; path . to_path_buf () }) ; if we_set { Ok (()) } else { Err (val . to_owned ()) } }
};
}
