// Generated macro for Entry (enum)
macro_rules! Depcrate_cursorEntry {
() => {
// Module: crate::cursor
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " Internal type which is used instead of `TokenTree` to represent a single"] # [doc = " `TokenTree` within a `SynomBuffer`."] # [derive (Debug)] enum Entry { # [doc = " Mimicing types from proc-macro."] Group (Span , Delimiter , SynomBuffer) , Term (Span , Term) , Op (Span , char , Spacing) , Literal (Span , Literal) , # [doc = " End entries contain a raw pointer to the entry from the containing"] # [doc = " TokenTree."] End (* const Entry) , }
};
}
