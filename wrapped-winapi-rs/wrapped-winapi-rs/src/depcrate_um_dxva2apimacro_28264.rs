// Generated macro for macro_28264 (macro)
macro_rules! Depcrate_um_dxva2apimacro_28264 {
() => {
// Module: crate::um::dxva2api
// Provides: {"macro_28264"}
// Dependencies: {}
RIDL ! { # [uuid (0xa0cade0f , 0x06d5 , 0x4cf4 , 0xa1 , 0xc7 , 0xf3 , 0xcd , 0xd7 , 0x25 , 0xaa , 0x75)] interface IDirect3DDeviceManager9 (IDirect3DDeviceManager9Vtbl) : IUnknown (IUnknownVtbl) { fn ResetDevice (pDevice : * mut IDirect3DDevice9 , resetToken : UINT ,) -> HRESULT , fn OpenDeviceHandle (phDevice : * mut HANDLE ,) -> HRESULT , fn CloseDeviceHandle (hDevice : HANDLE ,) -> HRESULT , fn TestDevice (hDevice : HANDLE ,) -> HRESULT , fn LockDevice (hDevice : HANDLE , ppDevice : * mut * mut IDirect3DDevice9 , fBloc : BOOL ,) -> HRESULT , fn UnlockDevice (hDevice : HANDLE , fSaveState : BOOL ,) -> HRESULT , fn GetVideoService (hDevice : HANDLE , riid : REFIID , ppService : * mut * mut c_void ,) -> HRESULT , } }
};
}
