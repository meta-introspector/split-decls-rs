// Generated macro for impl_611 (impl)
macro_rules! Depcrate_formattingimpl_611 {
() => {
// Module: crate::formatting
// Provides: {"impl_611"}
// Dependencies: {}
impl FormattingError { pub (crate) fn from_span (span : Span , psess : & ParseSess , kind : ErrorKind) -> FormattingError { FormattingError { line : psess . line_of_byte_pos (span . lo ()) , is_comment : kind . is_comment () , kind , is_string : false , line_buffer : psess . span_to_first_line_string (span) , } } pub (crate) fn is_internal (& self) -> bool { match self . kind { ErrorKind :: LineOverflow (..) | ErrorKind :: TrailingWhitespace | ErrorKind :: IoError (_) | ErrorKind :: ParseError | ErrorKind :: LostComment => true , _ => false , } } pub (crate) fn msg_suffix (& self) -> & str { if self . is_comment || self . is_string { "set `error_on_unformatted = false` to suppress \
             the warning against comments or string literals\n" } else { "" } } pub (crate) fn format_len (& self) -> (usize , usize) { match self . kind { ErrorKind :: LineOverflow (found , max) => (max , found - max) , ErrorKind :: TrailingWhitespace | ErrorKind :: DeprecatedAttr | ErrorKind :: BadAttr | ErrorKind :: LostComment => { let trailing_ws_start = self . line_buffer . rfind (| c : char | ! c . is_whitespace ()) . map (| pos | pos + 1) . unwrap_or (0) ; (trailing_ws_start , self . line_buffer . len () - trailing_ws_start ,) } _ => unreachable ! () , } } }
};
}
