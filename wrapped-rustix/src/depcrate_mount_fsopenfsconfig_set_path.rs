// Generated macro for fsconfig_set_path (function)
macro_rules! Depcrate_mount_fsopenfsconfig_set_path {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsconfig_set_path"}
// Dependencies: {}
# [doc = " `fsconfig(fs_fd, FSCONFIG_SET_PATH, key, path, fd)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_path < Key : path :: Arg , Path : path :: Arg , Fd : AsFd , AuxFd : AsFd > (fs_fd : Fd , key : Key , path : Path , fd : AuxFd ,) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; let fd = fd . as_fd () ; key . into_with_c_str (| key | { path . into_with_c_str (| path | { backend :: mount :: syscalls :: fsconfig_set_path (fs_fd , key , path , fd) }) }) }
};
}
