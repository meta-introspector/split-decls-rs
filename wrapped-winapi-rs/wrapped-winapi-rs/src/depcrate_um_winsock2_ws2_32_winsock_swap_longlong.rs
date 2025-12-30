// Generated macro for _WS2_32_WINSOCK_SWAP_LONGLONG (function)
macro_rules! Depcrate_um_winsock2_WS2_32_WINSOCK_SWAP_LONGLONG {
() => {
// Module: crate::um::winsock2
// Provides: {"_WS2_32_WINSOCK_SWAP_LONGLONG"}
// Dependencies: {}
# [inline] pub fn _WS2_32_WINSOCK_SWAP_LONGLONG (l : __uint64) -> __uint64 { ((l >> 56) & 0x00000000000000FF) | ((l >> 40) & 0x000000000000FF00) | ((l >> 24) & 0x0000000000FF0000) | ((l >> 8) & 0x00000000FF000000) | ((l << 8) & 0x000000FF00000000) | ((l << 24) & 0x0000FF0000000000) | ((l << 40) & 0x00FF000000000000) | ((l << 56) & 0xFF00000000000000) }
};
}
