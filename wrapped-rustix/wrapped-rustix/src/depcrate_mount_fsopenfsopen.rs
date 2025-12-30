// Generated macro for fsopen (function)
macro_rules! Depcrate_mount_fsopenfsopen {
() => {
// Module: crate::mount::fsopen
// Provides: {"fsopen"}
// Dependencies: {}
# [doc = " `fsopen(fs_name, flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsopen.md"] # [inline] pub fn fsopen < Fs : path :: Arg > (fs_name : Fs , flags : FsOpenFlags) -> io :: Result < OwnedFd > { fs_name . into_with_c_str (| fs_name | backend :: mount :: syscalls :: fsopen (fs_name , flags)) }
};
}
