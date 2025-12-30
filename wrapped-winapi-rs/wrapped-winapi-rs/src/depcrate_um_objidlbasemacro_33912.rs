// Generated macro for macro_33912 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33912 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33912"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000013d , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IClientSecurity (IClientSecurityVtbl) : IUnknown (IUnknownVtbl) { fn QueryBlanket (pProxy : * mut IUnknown , pAuthnSvc : * mut DWORD , pAuthzSvc : * mut DWORD , pServerPrincName : * mut * mut OLECHAR , pAuthnLevel : * mut DWORD , pImpLevel : * mut DWORD , pAuthInfo : * mut * mut c_void , pCapabilities : * mut DWORD ,) -> HRESULT , fn SetBlanket (pProxy : * mut IUnknown , dwAuthnSvc : DWORD , dwAuthzSvc : DWORD , pServerPrincName : * mut OLECHAR , dwAuthnLevel : DWORD , dwImpLevel : DWORD , pAuthInfo : * mut c_void , dwCapabilities : DWORD ,) -> HRESULT , fn CopyProxy (pProxy : * mut IUnknown , ppCopy : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
