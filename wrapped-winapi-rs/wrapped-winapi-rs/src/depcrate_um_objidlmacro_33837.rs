// Generated macro for macro_33837 (macro)
macro_rules! Depcrate_um_objidlmacro_33837 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33837"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000010b , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IPersistFile (IPersistFileVtbl) : IPersist (IPersistVtbl) { fn IsDirty () -> HRESULT , fn Load (pszFileName : LPCOLESTR , dwMode : DWORD ,) -> HRESULT , fn Save (pszFileName : LPCOLESTR , fRemember : BOOL ,) -> HRESULT , fn SaveCompleted (pszFileName : LPCOLESTR ,) -> HRESULT , fn GetCurFile (ppszFileName : * mut LPOLESTR ,) -> HRESULT , } }
};
}
