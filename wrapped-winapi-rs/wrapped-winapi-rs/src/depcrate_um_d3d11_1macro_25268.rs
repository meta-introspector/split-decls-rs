// Generated macro for macro_25268 (macro)
macro_rules! Depcrate_um_d3d11_1macro_25268 {
() => {
// Module: crate::um::d3d11_1
// Provides: {"macro_25268"}
// Dependencies: {}
RIDL ! { # [uuid (0xa04bfb29 , 0x08ef , 0x43d6 , 0xa4 , 0x9c , 0xa9 , 0xbd , 0xbd , 0xcb , 0xe6 , 0x86)] interface ID3D11Device1 (ID3D11Device1Vtbl) : ID3D11Device (ID3D11DeviceVtbl) { fn GetImmediateContext1 (ppImmediateContext : * mut * mut ID3D11DeviceContext1 ,) -> () , fn CreateDeferredContext1 (ContextFlags : UINT , ppDeferredContext : * mut * mut ID3D11DeviceContext1 ,) -> HRESULT , fn CreateBlendState (pBlendStateDesc : * const D3D11_BLEND_DESC1 , ppBlendState : * mut * mut ID3D11BlendState1 ,) -> HRESULT , fn CreateRasterizerState (pRasterizerDesc : * const D3D11_RASTERIZER_DESC1 , ppRasterizerState : * mut * mut ID3D11RasterizerState1 ,) -> HRESULT , fn CreateDeviceContextState (Flags : UINT , pFeatureLevels : * const D3D_FEATURE_LEVEL , FeatureLevels : UINT , SDKVersion : UINT , EmulatedInterface : REFIID , pChosenFeatureLevel : * mut D3D_FEATURE_LEVEL , ppContextState : * mut * mut ID3DDeviceContextState ,) -> HRESULT , fn OpenSharedResource1 (hResource : HANDLE , returnedInterface : REFIID , ppResource : * mut * mut c_void ,) -> HRESULT , fn OpenSharedResourceByName (Name : LPCWSTR , dwDesiredAccess : DWORD , returnedInterface : REFIID , ppResource : * mut * mut c_void ,) -> HRESULT , } }
};
}
