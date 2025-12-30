// Generated macro for symlink (function)
macro_rules! Depcrate_os_wasi_fssymlink {
() => {
// Module: crate::os::wasi::fs
// Provides: {"symlink"}
// Dependencies: {}
# [doc = " Creates a symbolic link."] # [doc = ""] # [doc = " This corresponds to the `path_symlink` syscall."] # [doc (alias = "path_symlink")] pub fn symlink < P : AsRef < Path > , U : AsRef < Path > > (old_path : P , fd : & File , new_path : U ,) -> io :: Result < () > { fd . as_inner () . as_inner () . symlink (osstr2str (old_path . as_ref () . as_ref ()) ? , osstr2str (new_path . as_ref () . as_ref ()) ?) }
};
}
