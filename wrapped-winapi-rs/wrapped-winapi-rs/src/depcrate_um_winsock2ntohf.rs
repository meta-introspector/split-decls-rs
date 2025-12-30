// Generated macro for ntohf (function)
macro_rules! Depcrate_um_winsock2ntohf {
() => {
// Module: crate::um::winsock2
// Provides: {"ntohf"}
// Dependencies: {}
# [inline] pub fn ntohf (Value : __uint32) -> c_float { let Tempval = _WS2_32_WINSOCK_SWAP_LONG (Value) ; unsafe { :: core :: mem :: transmute (Tempval) } }
};
}
