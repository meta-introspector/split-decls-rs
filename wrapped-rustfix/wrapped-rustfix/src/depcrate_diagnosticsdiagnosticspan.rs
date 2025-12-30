// Generated macro for DiagnosticSpan (struct)
macro_rules! Depcrate_diagnosticsDiagnosticSpan {
() => {
// Module: crate::diagnostics
// Provides: {"DiagnosticSpan"}
// Dependencies: {}
# [doc = " Span information of a diagnostic item."] # [derive (Clone , Deserialize , Debug , Hash , Eq , PartialEq)] pub struct DiagnosticSpan { pub file_name : String , pub byte_start : u32 , pub byte_end : u32 , # [doc = " 1-based."] pub line_start : usize , pub line_end : usize , # [doc = " 1-based, character offset."] pub column_start : usize , pub column_end : usize , # [doc = " Is this a \"primary\" span -- meaning the point, or one of the points,"] # [doc = " where the error occurred?"] pub is_primary : bool , # [doc = " Source text from the start of `line_start` to the end of `line_end`."] pub text : Vec < DiagnosticSpanLine > , # [doc = " Label that should be placed at this location (if any)"] label : Option < String > , # [doc = " If we are suggesting a replacement, this will contain text"] # [doc = " that should be sliced in atop this span."] pub suggested_replacement : Option < String > , # [doc = " If the suggestion is approximate"] pub suggestion_applicability : Option < Applicability > , # [doc = " Macro invocations that created the code at this span, if any."] expansion : Option < Box < DiagnosticSpanMacroExpansion > > , }
};
}
