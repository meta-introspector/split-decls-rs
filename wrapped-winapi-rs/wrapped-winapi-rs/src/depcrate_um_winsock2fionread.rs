// Generated macro for FIONREAD (const)
macro_rules! Depcrate_um_winsock2FIONREAD {
() => {
// Module: crate::um::winsock2
// Provides: {"FIONREAD"}
// Dependencies: {}
pub const FIONREAD : c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x66 << 8) | 127 ;
};
}
