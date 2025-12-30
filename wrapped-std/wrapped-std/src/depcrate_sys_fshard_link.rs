// Generated macro for hard_link (function)
macro_rules! Depcrate_sys_fshard_link {
() => {
// Module: crate::sys::fs
// Provides: {"hard_link"}
// Dependencies: {}
pub fn hard_link (original : & Path , link : & Path) -> io :: Result < () > { with_native_path (original , & | original | { with_native_path (link , & | link | imp :: link (original , link)) }) }
};
}
