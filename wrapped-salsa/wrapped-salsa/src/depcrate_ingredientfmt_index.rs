// Generated macro for fmt_index (function)
macro_rules! Depcrate_ingredientfmt_index {
() => {
// Module: crate::ingredient
// Provides: {"fmt_index"}
// Dependencies: {}
# [doc = " A helper function to show human readable fmt."] pub (crate) fn fmt_index (debug_name : & str , id : Id , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{debug_name}({id:?})") }
};
}
