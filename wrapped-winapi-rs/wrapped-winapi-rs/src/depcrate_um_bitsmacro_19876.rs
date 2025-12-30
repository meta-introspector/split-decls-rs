// Generated macro for macro_19876 (macro)
macro_rules! Depcrate_um_bitsmacro_19876 {
() => {
// Module: crate::um::bits
// Provides: {"macro_19876"}
// Dependencies: {}
RIDL ! { # [uuid (0x19c613a0 , 0xfcb8 , 0x4f28 , 0x81 , 0xae , 0x89 , 0x7c , 0x3d , 0x07 , 0x8f , 0x81)] interface IBackgroundCopyError (IBackgroundCopyErrorVtbl) : IUnknown (IUnknownVtbl) { fn GetError (pContext : * mut BG_ERROR_CONTEXT , pCode : * mut HRESULT ,) -> HRESULT , fn GetFile (pVal : * mut * mut IBackgroundCopyFile ,) -> HRESULT , fn GetErrorDescription (LanguageId : DWORD , pErrorDescription : * mut LPWSTR ,) -> HRESULT , fn GetErrorContextDescription (LanguageId : DWORD , pContextDescription : * mut LPWSTR ,) -> HRESULT , fn GetProtocol (pProtocol : * mut LPWSTR ,) -> HRESULT , } }
};
}
