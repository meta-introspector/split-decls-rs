// Generated macro for macro_2107 (macro)
macro_rules! Depcrate_shared_d3d9macro_2107 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2107"}
// Dependencies: {}
RIDL ! { # [uuid (0x02177241 , 0x69fc , 0x400c , 0x8f , 0xf1 , 0x93 , 0xa4 , 0x4d , 0xf6 , 0x86 , 0x1d)] interface IDirect3D9Ex (IDirect3D9ExVtbl) : IDirect3D9 (IDirect3D9Vtbl) { fn GetAdapterModeCountEx (Adapter : UINT , pFilter : * const D3DDISPLAYMODEFILTER ,) -> UINT , fn EnumAdapterModesEx (Adapter : UINT , pFilter : * const D3DDISPLAYMODEFILTER , Mode : UINT , pMode : * mut D3DDISPLAYMODEEX ,) -> HRESULT , fn GetAdapterDisplayModeEx (Adapter : UINT , pMode : * mut D3DDISPLAYMODEEX , pRotation : * mut D3DDISPLAYROTATION ,) -> HRESULT , fn CreateDeviceEx (Adapter : UINT , DeviceType : D3DDEVTYPE , hFocusWindow : HWND , BehaviorFlags : DWORD , pPresentationParameters : * mut D3DPRESENT_PARAMETERS , pFullscreenDisplayMode : * mut D3DDISPLAYMODEEX , ppReturnedDeviceInterface : * mut * mut IDirect3DDevice9Ex ,) -> HRESULT , fn GetAdapterLUID (Adapter : UINT , pLUID : * mut LUID ,) -> HRESULT , } }
};
}
