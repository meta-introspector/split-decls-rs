// Generated macro for other_39172 (other)
macro_rules! Depcrate_um_usp10other_39172 {
() => {
// Module: crate::um::usp10
// Provides: {"other_39172"}
// Dependencies: {}
extern "system" { pub fn ScriptBreak (pwcChars : * const WCHAR , cChars : c_int , psa : * const SCRIPT_ANALYSIS , psla : * mut SCRIPT_LOGATTR ,) -> HRESULT ; pub fn ScriptCPtoX (iCP : c_int , fTrailing : BOOL , cChars : c_int , cGlyphs : c_int , pwLogClust : * const WORD , psva : * const SCRIPT_VISATTR , piAdvance : * const c_int , psa : * const SCRIPT_ANALYSIS , piX : * mut c_int ,) -> HRESULT ; pub fn ScriptXtoCP (iX : c_int , cChars : c_int , cGlyphs : c_int , pwLogClust : * const WORD , psva : * const SCRIPT_VISATTR , piAdvance : * const c_int , psa : * const SCRIPT_ANALYSIS , piCP : * mut c_int , piTrailing : * mut c_int ,) -> HRESULT ; pub fn ScriptGetLogicalWidths (psa : * const SCRIPT_ANALYSIS , cChars : c_int , cGlyphs : c_int , piGlyphWidth : * const c_int , pwLogClust : * const WORD , psva : * const SCRIPT_VISATTR , piDx : * mut c_int ,) -> HRESULT ; pub fn ScriptApplyLogicalWidth (piDx : * const c_int , cChars : c_int , cGlyphs : c_int , pwLogClust : * const WORD , psva : * const SCRIPT_VISATTR , piAdvance : * const c_int , psa : * const SCRIPT_ANALYSIS , pABC : * mut ABC , piJustify : * mut c_int ,) -> HRESULT ; }
};
}
