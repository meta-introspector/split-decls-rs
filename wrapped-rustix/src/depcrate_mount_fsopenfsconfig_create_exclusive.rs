// Generated macro for fsconfig_create_exclusive (function)
macro_rules! Depcrate_mount_fsopenfsconfig_create_exclusive {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsconfig_create_exclusive"}
// Dependencies: {}
# [doc = " `fsconfig(fs_fd, FSCONFIG_CMD_CREATE_EXCL, key, NULL, 0)`"] # [doc = ""] # [doc = " This function was added in Linux 6.6."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_create_exclusive < Fd : AsFd > (fs_fd : Fd) -> io :: Result < () > { backend :: mount :: syscalls :: fsconfig_create_excl (fs_fd . as_fd ()) }
};
}
