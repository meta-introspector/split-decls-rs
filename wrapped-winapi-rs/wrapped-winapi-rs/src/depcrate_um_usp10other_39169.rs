// Generated macro for other_39169 (other)
macro_rules! Depcrate_um_usp10other_39169 {
() => {
// Module: crate::um::usp10
// Provides: {"other_39169"}
// Dependencies: {}
extern "system" { pub fn ScriptPlace (hdc : HDC , psc : * mut SCRIPT_CACHE , pwGlyphs : * const WORD , cGlyphs : c_int , psva : * const SCRIPT_VISATTR , psa : * mut SCRIPT_ANALYSIS , piAdvance : * mut c_int , pGoffset : * mut GOFFSET , pABC : * mut ABC ,) -> HRESULT ; pub fn ScriptTextOut (hdc : HDC , psc : * mut SCRIPT_CACHE , x : c_int , y : c_int , fuOptions : UINT , lprc : * const RECT , psa : * const SCRIPT_ANALYSIS , pwcReserved : * const WCHAR , iReserved : c_int , pwGlyphs : * const WORD , cGlyphs : c_int , piAdvance : * const c_int , piJustify : * const c_int , pGoffset : * const GOFFSET ,) -> HRESULT ; pub fn ScriptJustify (psva : * const SCRIPT_VISATTR , piAdvance : * const c_int , cGlyphs : c_int , iDx : c_int , iMinKashida : c_int , piJustify : * mut c_int ,) -> HRESULT ; }
};
}
