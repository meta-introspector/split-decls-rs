// Generated macro for SIOCGLOWAT (const)
macro_rules! Depcrate_um_winsock2SIOCGLOWAT {
() => {
// Module: crate::um::winsock2
// Provides: {"SIOCGLOWAT"}
// Dependencies: {}
pub const SIOCGLOWAT : c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 3 ;
};
}
