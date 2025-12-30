// Generated macro for macro_32021 (macro)
macro_rules! Depcrate_um_mmdeviceapimacro_32021 {
() => {
// Module: crate::um::mmdeviceapi
// Provides: {"macro_32021"}
// Dependencies: {}
RIDL ! { # [uuid (0xd666063f , 0x1587 , 0x4e43 , 0x81 , 0xf1 , 0xb9 , 0x48 , 0xe8 , 0x07 , 0x36 , 0x3f)] interface IMMDevice (IMMDeviceVtbl) : IUnknown (IUnknownVtbl) { fn Activate (iid : REFIID , dwClsCtx : DWORD , pActivationParams : * mut PROPVARIANT , ppInterface : * mut LPVOID ,) -> HRESULT , fn OpenPropertyStore (stgmAccess : DWORD , ppProperties : * mut * mut IPropertyStore ,) -> HRESULT , fn GetId (ppstrId : * mut LPWSTR ,) -> HRESULT , fn GetState (pdwState : * mut DWORD ,) -> HRESULT , } }
};
}
