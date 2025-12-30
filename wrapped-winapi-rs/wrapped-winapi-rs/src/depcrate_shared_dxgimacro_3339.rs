// Generated macro for macro_3339 (macro)
macro_rules! Depcrate_shared_dxgimacro_3339 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3339"}
// Dependencies: {}
RIDL ! { # [uuid (0x7b7166ec , 0x21c7 , 0x44ae , 0xb2 , 0x1a , 0xc9 , 0xae , 0x32 , 0x1a , 0xe3 , 0x69)] interface IDXGIFactory (IDXGIFactoryVtbl) : IDXGIObject (IDXGIObjectVtbl) { fn EnumAdapters (Adapter : UINT , ppAdapter : * mut * mut IDXGIAdapter ,) -> HRESULT , fn MakeWindowAssociation (WindowHandle : HWND , Flags : UINT ,) -> HRESULT , fn GetWindowAssociation (pWindowHandle : * mut HWND ,) -> HRESULT , fn CreateSwapChain (pDevice : * mut IUnknown , pDesc : * mut DXGI_SWAP_CHAIN_DESC , ppSwapChain : * mut * mut IDXGISwapChain ,) -> HRESULT , fn CreateSoftwareAdapter (Module : HMODULE , ppAdapter : * mut * mut IDXGIAdapter ,) -> HRESULT , } }
};
}
