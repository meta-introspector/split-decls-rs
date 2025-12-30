// Generated macro for get_glob (function)
macro_rules! Depcrate_specget_glob {
() => {
// Module: crate::spec
// Provides: {"get_glob"}
// Dependencies: {}
fn get_glob (path : & std :: path :: Path) -> Option < & str > { if let Some (utf8) = path . to_str () { if utf8 . contains ('*') { return Some (utf8) ; } } None }
};
}
