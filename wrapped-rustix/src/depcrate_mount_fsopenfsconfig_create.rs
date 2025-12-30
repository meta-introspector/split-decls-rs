// Generated macro for fsconfig_create (function)
macro_rules! Depcrate_mount_fsopenfsconfig_create {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsconfig_create"}
// Dependencies: {}
# [doc = " `fsconfig(fs_fd, FSCONFIG_CMD_CREATE, key, NULL, 0)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_create < Fd : AsFd > (fs_fd : Fd) -> io :: Result < () > { backend :: mount :: syscalls :: fsconfig_create (fs_fd . as_fd ()) }
};
}
