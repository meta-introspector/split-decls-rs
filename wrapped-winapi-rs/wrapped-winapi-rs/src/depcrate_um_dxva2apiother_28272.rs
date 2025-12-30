// Generated macro for other_28272 (other)
macro_rules! Depcrate_um_dxva2apiother_28272 {
() => {
// Module: crate::um::dxva2api
// Provides: {"other_28272"}
// Dependencies: {}
extern "system" { pub fn DXVA2CreateDirect3DDeviceManager9 (pResetToken : * mut UINT , ppDeviceManager : * mut * mut IDirect3DDeviceManager9 ,) -> HRESULT ; pub fn DXVA2CreateVideoService (pDD : * mut IDirect3DDevice9 , riid : REFIID , ppService : * mut * mut c_void ,) -> HRESULT ; }
};
}
