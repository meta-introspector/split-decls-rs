// Generated macro for SECURITY_MAX_SID_SIZE (const)
macro_rules! Depcrate_um_winntSECURITY_MAX_SID_SIZE {
() => {
// Module: crate::um::winnt
// Provides: {"SECURITY_MAX_SID_SIZE"}
// Dependencies: {}
pub const SECURITY_MAX_SID_SIZE : usize = 12 - 4 + (SID_MAX_SUB_AUTHORITIES as usize * 4) ;
};
}
