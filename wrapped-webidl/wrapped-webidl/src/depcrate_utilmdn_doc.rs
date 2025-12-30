// Generated macro for mdn_doc (function)
macro_rules! Depcrate_utilmdn_doc {
() => {
// Module: crate::util
// Provides: {"mdn_doc"}
// Dependencies: {}
pub fn mdn_doc (class : & str , method : Option < & str >) -> String { let mut link = format ! ("https://developer.mozilla.org/en-US/docs/Web/API/{class}") ; if let Some (method) = method { link . push_str (& format ! ("/{method}")) ; } format ! ("[MDN Documentation]({link})") }
};
}
