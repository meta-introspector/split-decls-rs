// Generated macro for open_tree (function)
macro_rules! Depcrate_mount_fsopenopen_tree {
() => {
// Module: crate::mount::fsopen
// Provides: {"open_tree"}
// Dependencies: {}
# [doc = " `open_tree(dfd, filename, flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/open_tree.md"] # [inline] pub fn open_tree < Path : path :: Arg , Fd : AsFd > (dfd : Fd , filename : Path , flags : OpenTreeFlags ,) -> io :: Result < OwnedFd > { let dfd = dfd . as_fd () ; filename . into_with_c_str (| filename | backend :: mount :: syscalls :: open_tree (dfd , filename , flags)) }
};
}
