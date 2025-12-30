// Generated macro for macro_3463 (macro)
macro_rules! Depcrate_shared_dxgi1_4macro_3463 {
() => {
// Module: crate::shared::dxgi1_4
// Provides: {"macro_3463"}
// Dependencies: {}
RIDL ! { # [uuid (0x1bc6ea02 , 0xef36 , 0x464f , 0xbf , 0x0c , 0x21 , 0xca , 0x39 , 0xe5 , 0x16 , 0x8a)] interface IDXGIFactory4 (IDXGIFactory4Vtbl) : IDXGIFactory3 (IDXGIFactory3Vtbl) { fn EnumAdapterByLuid (AdapterLuid : LUID , riid : REFGUID , ppvAdapter : * mut * mut c_void ,) -> HRESULT , fn EnumWarpAdapter (riid : REFGUID , ppvAdapter : * mut * mut c_void ,) -> HRESULT , } }
};
}
