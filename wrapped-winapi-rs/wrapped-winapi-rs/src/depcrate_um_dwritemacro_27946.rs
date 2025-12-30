// Generated macro for macro_27946 (macro)
macro_rules! Depcrate_um_dwritemacro_27946 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27946"}
// Dependencies: {}
RIDL ! { # [uuid (0x5810cd44 , 0x0ca0 , 0x4701 , 0xb3 , 0xfa , 0xbe , 0xc5 , 0x18 , 0x2a , 0xe4 , 0xf6)] interface IDWriteTextAnalysisSink (IDWriteTextAnalysisSinkVtbl) : IUnknown (IUnknownVtbl) { fn SetScriptAnalysis (textPosition : UINT32 , textLength : UINT32 , scriptAnalysis : * const DWRITE_SCRIPT_ANALYSIS ,) -> HRESULT , fn SetLineBreakpoints (textPosition : UINT32 , textLength : UINT32 , lineBreakpoints : * const DWRITE_LINE_BREAKPOINT ,) -> HRESULT , fn SetBidiLevel (textPosition : UINT32 , textLength : UINT32 , explicitLevel : UINT8 , resolvedLevel : UINT8 ,) -> HRESULT , fn SetNumberSubstitution (textPosition : UINT32 , textLength : UINT32 , numberSubstitution : * mut IDWriteNumberSubstitution ,) -> HRESULT , } }
};
}
