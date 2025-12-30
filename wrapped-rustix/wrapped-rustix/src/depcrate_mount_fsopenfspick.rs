// Generated macro for fspick (function)
macro_rules! Depcrate_mount_fsopenfspick {
() => {
// Module: crate::mount::fsopen
// Provides: {"fspick"}
// Dependencies: {}
# [doc = " `fspick(dfd, path, flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fspick.md"] # [inline] pub fn fspick < Path : path :: Arg , Fd : AsFd > (dfd : Fd , path : Path , flags : FsPickFlags ,) -> io :: Result < OwnedFd > { let dfd = dfd . as_fd () ; path . into_with_c_str (| path | backend :: mount :: syscalls :: fspick (dfd , path , flags)) }
};
}
