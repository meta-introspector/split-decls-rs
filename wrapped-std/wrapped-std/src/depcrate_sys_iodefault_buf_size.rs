// Generated macro for DEFAULT_BUF_SIZE (const)
macro_rules! Depcrate_sys_ioDEFAULT_BUF_SIZE {
() => {
// Module: crate::sys::io
// Provides: {"DEFAULT_BUF_SIZE"}
// Dependencies: {}
pub const DEFAULT_BUF_SIZE : usize = if cfg ! (target_os = "espidf") { 512 } else { 8 * 1024 } ;
};
}
