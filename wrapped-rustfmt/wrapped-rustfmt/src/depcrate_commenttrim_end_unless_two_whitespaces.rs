// Generated macro for trim_end_unless_two_whitespaces (function)
macro_rules! Depcrate_commenttrim_end_unless_two_whitespaces {
() => {
// Module: crate::comment
// Provides: {"trim_end_unless_two_whitespaces"}
// Dependencies: {}
# [doc = " Trim trailing whitespaces unless they consist of two or more whitespaces."] fn trim_end_unless_two_whitespaces (s : & str , is_doc_comment : bool) -> & str { if is_doc_comment && s . ends_with ("  ") { s } else { s . trim_end () } }
};
}
