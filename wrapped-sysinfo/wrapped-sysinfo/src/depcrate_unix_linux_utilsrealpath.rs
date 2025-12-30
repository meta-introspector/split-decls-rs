// Generated macro for realpath (function)
macro_rules! Depcrate_unix_linux_utilsrealpath {
() => {
// Module: crate::unix::linux::utils
// Provides: {"realpath"}
// Dependencies: {}
# [cfg (feature = "system")] # [allow (clippy :: useless_conversion)] pub (crate) fn realpath (path : & Path) -> Option < std :: path :: PathBuf > { match std :: fs :: read_link (path) { Ok (path) => Some (path) , Err (_e) => { sysinfo_debug ! ("failed to get real path for {:?}: {:?}" , path , _e) ; None } } }
};
}
