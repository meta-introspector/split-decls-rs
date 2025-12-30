// Generated macro for macro_39816 (macro)
macro_rules! Depcrate_um_wbemtranmacro_39816 {
() => {
// Module: crate::um::wbemtran
// Provides: {"macro_39816"}
// Dependencies: {}
RIDL ! { # [uuid (0xa889c72a , 0xfcc1 , 0x4a9e , 0xaf , 0x61 , 0xed , 0x07 , 0x13 , 0x33 , 0xfb , 0x5b)] interface IWbemClientConnectionTransport (IWbemClientConnectionTransportVtbl) : IUnknown (IUnknownVtbl) { fn Open (strAddressType : BSTR , dwBinaryAddressLength : DWORD , abBinaryAddress : * mut BYTE , strObject : BSTR , strUser : BSTR , strPassword : BSTR , strLocale : BSTR , lFlags : c_long , pCtx : * mut IWbemContext , riid : REFIID , pInterface : * mut * mut c_void , pCallRes : * mut * mut IWbemCallResult ,) -> HRESULT , fn OpenAsync (strAddressType : BSTR , dwBinaryAddressLength : DWORD , abBinaryAddress : * mut BYTE , strObject : BSTR , strUser : BSTR , strPassword : BSTR , strLocale : BSTR , lFlags : c_long , pCtx : * mut IWbemContext , riid : REFIID , pResponseHandler : * mut IWbemObjectSink ,) -> HRESULT , fn Cancel (lFlags : c_long , pHandler : * mut IWbemObjectSink ,) -> HRESULT , } }
};
}
