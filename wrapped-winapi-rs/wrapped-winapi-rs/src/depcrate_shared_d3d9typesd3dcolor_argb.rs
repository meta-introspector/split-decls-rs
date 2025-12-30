// Generated macro for D3DCOLOR_ARGB (function)
macro_rules! Depcrate_shared_d3d9typesD3DCOLOR_ARGB {
() => {
// Module: crate::shared::d3d9types
// Provides: {"D3DCOLOR_ARGB"}
// Dependencies: {}
# [inline] pub fn D3DCOLOR_ARGB (a : DWORD , r : DWORD , g : DWORD , b : DWORD) -> D3DCOLOR { (((a & 0xff) << 24) | ((r & 0xff) << 16) | ((g & 0xff) << 8) | (b & 0xff)) as D3DCOLOR }
};
}
