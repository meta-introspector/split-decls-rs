// Generated macro for fclonefileat (function)
macro_rules! Depcrate_fs_atfclonefileat {
() => {
// Module: crate::fs::at
// Provides: {"fclonefileat"}
// Dependencies: {}
# [doc = " `fclonefileat(src, dst_dir, dst, flags)`—Efficiently copies between files."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://github.com/apple-oss-distributions/xnu/blob/main/bsd/man/man2/clonefile.2"] # [cfg (apple)] # [inline] pub fn fclonefileat < Fd : AsFd , DstFd : AsFd , P : path :: Arg > (src : Fd , dst_dir : DstFd , dst : P , flags : CloneFlags ,) -> io :: Result < () > { dst . into_with_c_str (| dst | { backend :: fs :: syscalls :: fclonefileat (src . as_fd () , dst_dir . as_fd () , dst , flags) }) }
};
}
