// Generated macro for macro_33913 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33913 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33913"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000013e , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IServerSecurity (IServerSecurityVtbl) : IUnknown (IUnknownVtbl) { fn QueryBlanket (pAuthnSvc : * mut DWORD , pAuthzSvc : * mut DWORD , pServerPrincName : * mut * mut OLECHAR , pAuthnLevel : * mut DWORD , pImpLevel : * mut DWORD , pPrivs : * mut * mut c_void , pCapabilities : * mut DWORD ,) -> HRESULT , fn ImpersonateClient () -> HRESULT , fn RevertToSelf () -> HRESULT , fn IsImpersonating () -> BOOL , } }
};
}
