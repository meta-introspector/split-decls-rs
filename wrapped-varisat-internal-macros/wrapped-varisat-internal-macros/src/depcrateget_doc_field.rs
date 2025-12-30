// Generated macro for get_doc_field (function)
macro_rules! Depcrateget_doc_field {
() => {
// Module: crate
// Provides: {"get_doc_field"}
// Dependencies: {}
# [doc = " Find a field inside the doc comment"] fn get_doc_field (name : & str , attrs : & [Attribute]) -> Option < LitStr > { let re = regex :: Regex :: new (& format ! (r"\[{}: (.+?)\](  |$)" , regex :: escape (name))) . unwrap () ; for doc_str in doc_from_attrs (attrs) { if let Some (expr_str) = re . captures (& doc_str . value ()) { let expr_str = expr_str . get (1) . unwrap () . as_str () ; let expr_str = LitStr :: new (expr_str , doc_str . span ()) ; return Some (expr_str) ; } } None }
};
}
