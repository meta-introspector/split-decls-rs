// Generated macro for WSADATA (struct)
macro_rules! Depcrate_b_archWSADATA {
() => {
// Module: crate::b_arch
// Provides: {"WSADATA"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct WSADATA { pub wVersion : u16 , pub wHighVersion : u16 , pub iMaxSockets : u16 , pub iMaxUdpDg : u16 , pub lpVendorInfo : PSTR , pub szDescription : [i8 ; 257] , pub szSystemStatus : [i8 ; 129] , }
};
}
