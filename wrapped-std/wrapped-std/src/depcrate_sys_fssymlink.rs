// Generated macro for symlink (function)
macro_rules! Depcrate_sys_fssymlink {
() => {
// Module: crate::sys::fs
// Provides: {"symlink"}
// Dependencies: {}
pub fn symlink (original : & Path , link : & Path) -> io :: Result < () > { # [cfg (windows)] return imp :: symlink (original , link) ; # [cfg (not (windows))] with_native_path (original , & | original | { with_native_path (link , & | link | imp :: symlink (original , link)) }) }
};
}
