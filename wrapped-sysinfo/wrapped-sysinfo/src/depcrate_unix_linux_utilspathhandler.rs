// Generated macro for PathHandler (struct)
macro_rules! Depcrate_unix_linux_utilsPathHandler {
() => {
// Module: crate::unix::linux::utils
// Provides: {"PathHandler"}
// Dependencies: {}
# [doc = " This type is used in `retrieve_all_new_process_info` because we have a \"parent\" path and"] # [doc = " from it, we `pop`/`join` every time because it's more memory efficient than using `Path::join`."] # [cfg (feature = "system")] pub (crate) struct PathHandler (std :: path :: PathBuf) ;
};
}
