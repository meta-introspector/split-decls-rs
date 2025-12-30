// Generated macro for macro_36838 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36838 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36838"}
// Dependencies: {}
RIDL ! { # [uuid (0xf8e690f0 , 0x39cb , 0x4843 , 0xb8 , 0xd7 , 0xc8 , 0x46 , 0x96 , 0xe1 , 0x11 , 0x9d)] interface ISpTokenUI (ISpTokenUIVtbl) : IUnknown (IUnknownVtbl) { fn IsUISupported (pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG , punkObject : * mut IUnknown , pfSupported : * mut BOOL ,) -> HRESULT , fn DisplayUI (hwndParent : HWND , pszTitle : LPCWSTR , pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG , pToken : * mut ISpObjectToken , punkObject : * mut IUnknown ,) -> HRESULT , } }
};
}
