// Generated macro for SIOCSLOWAT (const)
macro_rules! Depcrate_um_winsock2SIOCSLOWAT {
() => {
// Module: crate::um::winsock2
// Provides: {"SIOCSLOWAT"}
// Dependencies: {}
pub const SIOCSLOWAT : c_long = IOC_IN | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 2 ;
};
}
