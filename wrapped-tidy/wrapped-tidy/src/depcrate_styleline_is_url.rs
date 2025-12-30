// Generated macro for line_is_url (function)
macro_rules! Depcrate_styleline_is_url {
() => {
// Module: crate::style
// Provides: {"line_is_url"}
// Dependencies: {}
# [doc = " Returns `true` if `line` appears to be a line comment containing a URL,"] # [doc = " possibly with a Markdown link label in front, and nothing else."] # [doc = " The Markdown link label, if present, may not contain whitespace."] # [doc = " Lines of this form are allowed to be overlength, because Markdown"] # [doc = " offers no way to split a line in the middle of a URL, and the lengths"] # [doc = " of URLs to external references are beyond our control."] fn line_is_url (is_error_code : bool , columns : usize , line : & str) -> bool { if is_error_code { return line . starts_with ('[') && line . contains ("]:") && line . contains ("http") ; } use self :: LIUState :: * ; let mut state : LIUState = EXP_COMMENT_START ; let is_url = | w : & str | w . starts_with ("http://") || w . starts_with ("https://") ; for tok in line . split_whitespace () { match (state , tok) { (EXP_COMMENT_START , "//") | (EXP_COMMENT_START , "///") | (EXP_COMMENT_START , "//!") => { state = EXP_LINK_LABEL_OR_URL } (EXP_LINK_LABEL_OR_URL , w) if w . len () >= 4 && w . starts_with ('[') && w . ends_with ("]:") => { state = EXP_URL } (EXP_LINK_LABEL_OR_URL , w) if is_url (w) => state = EXP_END , (EXP_URL , w) if is_url (w) || w . starts_with ("../") => state = EXP_END , (_ , w) if w . len () > columns && is_url (w) => state = EXP_END , (_ , _) => { } } } state == EXP_END }
};
}
