// Generated macro for macro_36839 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36839 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36839"}
// Dependencies: {}
RIDL ! { # [uuid (0x06b64f9f , 0x7fda , 0x11d2 , 0xb4 , 0xf2 , 0x00 , 0xc0 , 0x4f , 0x79 , 0x73 , 0x96)] interface ISpObjectTokenEnumBuilder (ISpObjectTokenEnumBuilderVtbl) : IEnumSpObjectTokens (IEnumSpObjectTokensVtbl) { fn SetAttribs (pszReqAttribs : LPCWSTR , pszOptAttribs : LPCWSTR ,) -> HRESULT , fn AddTokens (cTokens : ULONG , pToken : * mut * mut ISpObjectToken ,) -> HRESULT , fn AddTokensFromDataKey (pDataKey : * mut ISpDataKey , pszSubKey : LPCWSTR , pszCategoryId : LPCWSTR ,) -> HRESULT , fn AddTokensFromTokenEnum (pTokenEnum : * mut IEnumSpObjectTokens ,) -> HRESULT , fn Sort (pszTokenIdToListFirst : LPCWSTR ,) -> HRESULT , } }
};
}
