// Generated macro for rename (function)
macro_rules! Depcrate_sys_fsrename {
() => {
// Module: crate::sys::fs
// Provides: {"rename"}
// Dependencies: {}
pub fn rename (old : & Path , new : & Path) -> io :: Result < () > { with_native_path (old , & | old | with_native_path (new , & | new | imp :: rename (old , new))) }
};
}
