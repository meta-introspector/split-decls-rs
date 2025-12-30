// Generated macro for fsconfig_reconfigure (function)
macro_rules! Depcrate_mount_fsopenfsconfig_reconfigure {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsconfig_reconfigure"}
// Dependencies: {}
# [doc = " `fsconfig(fs_fd, FSCONFIG_CMD_RECONFIGURE, key, NULL, 0)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_reconfigure < Fd : AsFd > (fs_fd : Fd) -> io :: Result < () > { backend :: mount :: syscalls :: fsconfig_reconfigure (fs_fd . as_fd ()) }
};
}
