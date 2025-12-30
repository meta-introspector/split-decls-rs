// Generated macro for Entry (enum)
macro_rules! Depcrate_bufferEntry {
() => {
// Module: crate::buffer
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " Internal type which is used instead of `TokenTree` to represent a token tree"] # [doc = " within a `TokenBuffer`."] enum Entry { Group (Group , usize) , Ident (Ident) , Punct (Punct) , Literal (Literal) , End (isize , isize) , }
};
}
