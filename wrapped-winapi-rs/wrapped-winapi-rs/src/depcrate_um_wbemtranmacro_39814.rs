// Generated macro for macro_39814 (macro)
macro_rules! Depcrate_um_wbemtranmacro_39814 {
() => {
// Module: crate::um::wbemtran
// Provides: {"macro_39814"}
// Dependencies: {}
RIDL ! { # [uuid (0xf7ce2e11 , 0x8c90 , 0x11d1 , 0x9e , 0x7b , 0x00 , 0xc0 , 0x4f , 0xc3 , 0x24 , 0xa8)] interface IWbemClientTransport (IWbemClientTransportVtbl) : IUnknown (IUnknownVtbl) { fn ConnectServer (strAddressType : BSTR , dwBinaryAddressLength : DWORD , abBinaryAddress : * mut BYTE , strNetworkResource : BSTR , strUser : BSTR , strPassword : BSTR , strLocale : BSTR , lSecurityFlags : c_long , strAuthority : BSTR , pCtx : * mut IWbemContext , ppNamespace : * mut * mut IWbemServices ,) -> HRESULT , } }
};
}
