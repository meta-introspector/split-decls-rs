// Generated macro for D3DCOLOR_COLORVALUE (function)
macro_rules! Depcrate_shared_d3d9typesD3DCOLOR_COLORVALUE {
() => {
// Module: crate::shared::d3d9types
// Provides: {"D3DCOLOR_COLORVALUE"}
// Dependencies: {}
# [inline] pub fn D3DCOLOR_COLORVALUE (r : f32 , g : f32 , b : f32 , a : f32) -> D3DCOLOR { D3DCOLOR_ARGB ((r * 255f32) as DWORD , (g * 255f32) as DWORD , (b * 255f32) as DWORD , (a * 255f32) as DWORD ,) }
};
}
