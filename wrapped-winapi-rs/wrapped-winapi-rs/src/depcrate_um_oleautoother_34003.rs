// Generated macro for other_34003 (other)
macro_rules! Depcrate_um_oleautoother_34003 {
() => {
// Module: crate::um::oleauto
// Provides: {"other_34003"}
// Dependencies: {}
extern "system" { pub fn LoadTypeLibEx (szFile : LPCOLESTR , regkind : REGKIND , pptlib : * mut * mut ITypeLib ,) -> HRESULT ; pub fn RevokeActiveObject (dwRegister : DWORD , pvReserved : * mut c_void ,) ; pub fn SetErrorInfo (dwReserved : ULONG , perrinfo : * mut IErrorInfo ,) -> HRESULT ; pub fn GetErrorInfo (dwReserved : ULONG , pperrinfo : * mut * mut IErrorInfo ,) -> HRESULT ; pub fn CreateErrorInfo (pperrinfo : * mut * mut ICreateErrorInfo ,) -> HRESULT ; pub fn OaBuildVersion () -> ULONG ; pub fn OaEnablePerUserTLibRegistration () ; }
};
}
