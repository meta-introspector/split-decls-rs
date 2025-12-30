// Generated macro for root_relative (function)
macro_rules! Depcrate_sys_platform_version_darwinroot_relative {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"root_relative"}
// Dependencies: {}
# [doc = " Get a path relative to the root directory in which all files for the current env are located."] fn root_relative (path : & str) -> Cow < '_ , Path > { if cfg ! (target_abi = "sim") { let mut root = PathBuf :: from (env :: var_os ("IPHONE_SIMULATOR_ROOT") . expect ("environment variable `IPHONE_SIMULATOR_ROOT` must be set when executing under simulator" ,)) ; root . push (Path :: new (path) . strip_prefix ("/") . unwrap ()) ; root . into () } else { Path :: new (path) . into () } }
};
}
