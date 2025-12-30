// Generated macro for long_line_is_ok (function)
macro_rules! Depcrate_stylelong_line_is_ok {
() => {
// Module: crate::style
// Provides: {"long_line_is_ok"}
// Dependencies: {}
# [doc = " Returns `true` if `line` is allowed to be longer than the normal limit."] fn long_line_is_ok (extension : & str , is_error_code : bool , max_columns : usize , line : & str) -> bool { match extension { "ftl" => true , "md" if ! is_error_code => true , "md" if line == INTERNAL_COMPILER_DOCS_LINE => true , _ => line_is_url (is_error_code , max_columns , line) || should_ignore (line) , } }
};
}
