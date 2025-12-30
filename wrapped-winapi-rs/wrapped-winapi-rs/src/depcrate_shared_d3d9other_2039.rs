// Generated macro for other_2039 (other)
macro_rules! Depcrate_shared_d3d9other_2039 {
() => {
// Module: crate::shared::d3d9
// Provides: {"other_2039"}
// Dependencies: {}
extern "system" { pub fn Direct3DCreate9 (SDKVersion : UINT ,) -> * mut IDirect3D9 ; pub fn D3DPERF_BeginEvent (col : D3DCOLOR , wszName : LPCWSTR ,) -> INT ; pub fn D3DPERF_EndEvent () -> INT ; pub fn D3DPERF_SetMarker (col : D3DCOLOR , wszName : LPCWSTR ,) -> () ; pub fn D3DPERF_SetRegion (col : D3DCOLOR , wszName : LPCWSTR ,) -> () ; pub fn D3DPERF_QueryRepeatFrame () -> BOOL ; pub fn D3DPERF_SetOptions (dwOptions : DWORD ,) -> () ; pub fn D3DPERF_GetStatus () -> DWORD ; }
};
}
