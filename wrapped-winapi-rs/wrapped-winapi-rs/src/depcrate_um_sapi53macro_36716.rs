// Generated macro for macro_36716 (macro)
macro_rules! Depcrate_um_sapi53macro_36716 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36716"}
// Dependencies: {}
RIDL ! { # [uuid (0x133adcd4 , 0x19b4 , 0x4020 , 0x9f , 0xdc , 0x84 , 0x2e , 0x78 , 0x25 , 0x3b , 0x17)] interface ISpPhoneticAlphabetConverter (ISpPhoneticAlphabetConverterVtbl) : IUnknown (IUnknownVtbl) { fn GetLangId (pLangID : * mut WORD ,) -> HRESULT , fn SetLangId (LangID : WORD ,) -> HRESULT , fn SAPI2UPS (pszSAPIId : * const SPPHONEID , pszUPSId : * mut SPPHONEID , cMaxLength : DWORD ,) -> HRESULT , fn UPS2SAPI (pszUPSId : * const SPPHONEID , pszSAPIId : * mut SPPHONEID , cMaxLength : DWORD ,) -> HRESULT , fn GetMaxConvertLength (cSrcLength : DWORD , bSAPI2UPS : BOOL , pcMaxDestLength : * mut DWORD ,) -> HRESULT , } }
};
}
