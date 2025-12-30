// Generated macro for htond (function)
macro_rules! Depcrate_um_winsock2htond {
() => {
// Module: crate::um::winsock2
// Provides: {"htond"}
// Dependencies: {}
# [inline] pub fn htond (Value : c_double) -> __uint64 { let Tempval : __uint64 = unsafe { :: core :: mem :: transmute (Value) } ; _WS2_32_WINSOCK_SWAP_LONGLONG (Tempval) }
};
}
