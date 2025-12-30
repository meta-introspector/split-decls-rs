// Generated macro for fsconfig_set_flag (function)
macro_rules! Depcrate_mount_fsopenfsconfig_set_flag {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsconfig_set_flag"}
// Dependencies: {}
# [doc = " `fsconfig(fs_fd, FSCONFIG_SET_FLAG, key, NULL, 0)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_flag < Key : path :: Arg , Fd : AsFd > (fs_fd : Fd , key : Key) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; key . into_with_c_str (| key | backend :: mount :: syscalls :: fsconfig_set_flag (fs_fd , key)) }
};
}
