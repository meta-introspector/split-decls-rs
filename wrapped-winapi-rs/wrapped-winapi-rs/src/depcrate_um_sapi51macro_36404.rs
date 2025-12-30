// Generated macro for macro_36404 (macro)
macro_rules! Depcrate_um_sapi51macro_36404 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36404"}
// Dependencies: {}
RIDL ! { # [uuid (0x2d3d3845 , 0x39af , 0x4850 , 0xbb , 0xf9 , 0x40 , 0xb4 , 0x97 , 0x80 , 0x01 , 0x1d)] interface ISpObjectTokenCategory (ISpObjectTokenCategoryVtbl) : ISpDataKey (ISpDataKeyVtbl) { fn SetId (pszCategoryId : LPCWSTR , fCreateIfNotExist : BOOL ,) -> HRESULT , fn GetId (ppszCoMemCategoryId : * mut LPWSTR ,) -> HRESULT , fn GetDataKey (spdkl : SPDATAKEYLOCATION , pppDataKey : * mut * mut ISpDataKey ,) -> HRESULT , fn EnumTokens (pzsReqAttribs : LPCWSTR , pszOptAttribs : LPCWSTR , ppEnum : * mut * mut IEnumSpObjectTokens ,) -> HRESULT , fn SetDefaultTokenId (pszTokenId : LPCWSTR ,) -> HRESULT , fn GetDefaultTokenId (ppszCoMemTokenId : * mut LPWSTR ,) -> HRESULT , } }
};
}
