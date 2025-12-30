// Generated macro for _WS2_32_WINSOCK_SWAP_LONG (function)
macro_rules! Depcrate_um_winsock2_WS2_32_WINSOCK_SWAP_LONG {
() => {
// Module: crate::um::winsock2
// Provides: {"_WS2_32_WINSOCK_SWAP_LONG"}
// Dependencies: {}
# [inline] pub fn _WS2_32_WINSOCK_SWAP_LONG (l : __uint32) -> __uint32 { ((l >> 24) & 0x000000FF) | ((l >> 8) & 0x0000FF00) | ((l << 8) & 0x00FF0000) | ((l << 24) & 0xFF000000) }
};
}
