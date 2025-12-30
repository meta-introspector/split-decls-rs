// Generated macro for htonf (function)
macro_rules! Depcrate_um_winsock2htonf {
() => {
// Module: crate::um::winsock2
// Provides: {"htonf"}
// Dependencies: {}
# [inline] pub fn htonf (Value : c_float) -> __uint32 { let Tempval : __uint32 = unsafe { :: core :: mem :: transmute (Value) } ; _WS2_32_WINSOCK_SWAP_LONG (Tempval) }
};
}
