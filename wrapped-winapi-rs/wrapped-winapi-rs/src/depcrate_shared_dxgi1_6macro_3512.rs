// Generated macro for macro_3512 (macro)
macro_rules! Depcrate_shared_dxgi1_6macro_3512 {
() => {
// Module: crate::shared::dxgi1_6
// Provides: {"macro_3512"}
// Dependencies: {}
RIDL ! { # [uuid (0xc1b6694f , 0xff09 , 0x44a9 , 0xb0 , 0x3c , 0x77 , 0x90 , 0x0a , 0x0a , 0x1d , 0x17)] interface IDXGIFactory6 (IDXGIFactory6Vtbl) : IDXGIFactory5 (IDXGIFactory5Vtbl) { fn EnumAdapterByGpuPreference (Adapter : UINT , GpuPreference : DXGI_GPU_PREFERENCE , riid : REFIID , ppvAdapter : * mut * mut c_void ,) -> HRESULT , } }
};
}
