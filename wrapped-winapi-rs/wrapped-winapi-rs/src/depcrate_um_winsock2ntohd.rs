// Generated macro for ntohd (function)
macro_rules! Depcrate_um_winsock2ntohd {
() => {
// Module: crate::um::winsock2
// Provides: {"ntohd"}
// Dependencies: {}
# [inline] pub fn ntohd (Value : __uint64) -> c_double { let Tempval = _WS2_32_WINSOCK_SWAP_LONGLONG (Value) ; unsafe { :: core :: mem :: transmute (Tempval) } }
};
}
