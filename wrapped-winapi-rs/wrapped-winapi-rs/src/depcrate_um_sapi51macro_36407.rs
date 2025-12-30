// Generated macro for macro_36407 (macro)
macro_rules! Depcrate_um_sapi51macro_36407 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36407"}
// Dependencies: {}
RIDL ! { # [uuid (0x06b64f9e , 0x7fda , 0x11d2 , 0xb4 , 0xf2 , 0x00 , 0xc0 , 0x4f , 0x79 , 0x73 , 0x96)] interface IEnumSpObjectTokens (IEnumSpObjectTokensVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , pelt : * mut * mut ISpObjectToken , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppEnum : * mut * mut IEnumSpObjectTokens ,) -> HRESULT , fn Item (Index : ULONG , ppToken : * mut * mut ISpObjectToken ,) -> HRESULT , fn GetCount (pCount : * mut ULONG ,) -> HRESULT , } }
};
}
