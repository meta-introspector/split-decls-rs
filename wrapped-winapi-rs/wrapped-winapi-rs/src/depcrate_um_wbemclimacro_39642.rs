// Generated macro for macro_39642 (macro)
macro_rules! Depcrate_um_wbemclimacro_39642 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39642"}
// Dependencies: {}
RIDL ! { # [uuid (0x6daf974e , 0x2e37 , 0x11d2 , 0xae , 0xc9 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IMofCompiler (IMofCompilerVtbl) : IUnknown (IUnknownVtbl) { fn CompileFile (FileName : LPWSTR , ServerAndNamespace : LPWSTR , User : LPWSTR , Authority : LPWSTR , Password : LPWSTR , lOptionFlags : LONG , lClassFlags : LONG , lInstanceFlags : LONG , pInfo : * mut WBEM_COMPILE_STATUS_INFO ,) -> HRESULT , fn CompileBuffer (BuffSize : c_long , pBuffer : * mut BYTE , ServerAndNamespace : LPWSTR , User : LPWSTR , Authority : LPWSTR , Password : LPWSTR , lOptionFlags : LONG , lClassFlags : LONG , lInstanceFlags : LONG , pInfo : * mut WBEM_COMPILE_STATUS_INFO ,) -> HRESULT , fn CreateBMOF (TextFileName : LPWSTR , BMOFFileName : LPWSTR , ServerAndNamespace : LPWSTR , lOptionFlags : LONG , lClassFlags : LONG , lInstanceFlags : LONG , pInfo : * mut WBEM_COMPILE_STATUS_INFO ,) -> HRESULT , } }
};
}
