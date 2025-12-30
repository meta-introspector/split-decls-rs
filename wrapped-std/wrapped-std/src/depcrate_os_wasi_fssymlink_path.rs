// Generated macro for symlink_path (function)
macro_rules! Depcrate_os_wasi_fssymlink_path {
() => {
// Module: crate::os::wasi::fs
// Provides: {"symlink_path"}
// Dependencies: {}
# [doc = " Creates a symbolic link."] # [doc = ""] # [doc = " This is a convenience API similar to `std::os::unix::fs::symlink` and"] # [doc = " `std::os::windows::fs::symlink_file` and `std::os::windows::fs::symlink_dir`."] pub fn symlink_path < P : AsRef < Path > , U : AsRef < Path > > (old_path : P , new_path : U) -> io :: Result < () > { crate :: sys :: fs :: symlink (old_path . as_ref () , new_path . as_ref ()) }
};
}
