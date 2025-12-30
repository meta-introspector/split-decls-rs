// Generated macro for other_39204 (other)
macro_rules! Depcrate_um_usp10other_39204 {
() => {
// Module: crate::um::usp10
// Provides: {"other_39204"}
// Dependencies: {}
extern "system" { pub fn ScriptStringAnalyse (hdc : HDC , pString : * const c_void , cString : c_int , cGlyphs : c_int , iCharset : c_int , dwFlags : DWORD , iReqWidth : c_int , psControl : * mut SCRIPT_CONTROL , psState : * mut SCRIPT_STATE , piDx : * const c_int , pTabdef : * mut SCRIPT_TABDEF , pbInClass : * const BYTE , pssa : * mut SCRIPT_STRING_ANALYSIS ,) -> HRESULT ; pub fn ScriptStringFree (pssa : * mut SCRIPT_STRING_ANALYSIS ,) -> HRESULT ; pub fn ScriptString_pSize (ssa : SCRIPT_STRING_ANALYSIS ,) -> * const SIZE ; pub fn ScriptString_pcOutChars (ssa : SCRIPT_STRING_ANALYSIS ,) -> * const c_int ; pub fn ScriptString_pLogAttr (ssa : SCRIPT_STRING_ANALYSIS ,) -> * const SCRIPT_LOGATTR ; pub fn ScriptStringGetOrder (ssa : SCRIPT_STRING_ANALYSIS , puOrder : * mut UINT ,) -> HRESULT ; pub fn ScriptStringCPtoX (ssa : SCRIPT_STRING_ANALYSIS , icp : c_int , fTrailing : BOOL , pX : * mut c_int ,) -> HRESULT ; pub fn ScriptStringXtoCP (ssa : SCRIPT_STRING_ANALYSIS , iX : c_int , piCh : * mut c_int , piTrailing : * mut c_int ,) -> HRESULT ; pub fn ScriptStringGetLogicalWidths (ssa : SCRIPT_STRING_ANALYSIS , dpiDx : * mut c_int ,) -> HRESULT ; pub fn ScriptStringValidate (ssa : SCRIPT_STRING_ANALYSIS ,) -> HRESULT ; pub fn ScriptStringOut (ssa : SCRIPT_STRING_ANALYSIS , iX : c_int , iY : c_int , uOptions : UINT , prc : * const RECT , iMinSel : c_int , iMaxSel : c_int , fDisabled : BOOL ,) -> HRESULT ; }
};
}
