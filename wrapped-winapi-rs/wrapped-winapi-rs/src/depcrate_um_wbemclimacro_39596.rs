// Generated macro for macro_39596 (macro)
macro_rules! Depcrate_um_wbemclimacro_39596 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39596"}
// Dependencies: {}
RIDL ! { # [uuid (0xdc12a687 , 0x737f , 0x11cf , 0x88 , 0x4d , 0x00 , 0xaa , 0x00 , 0x4b , 0x2e , 0x24)] interface IWbemLocator (IWbemLocatorVtbl) : IUnknown (IUnknownVtbl) { fn ConnectServer (strNetworkResource : BSTR , strUser : BSTR , strPassword : BSTR , strLocale : BSTR , lSecurityFlags : c_long , strAuthority : BSTR , pCtx : * mut IWbemContext , ppNamespace : * mut * mut IWbemServices ,) -> HRESULT , } }
};
}
