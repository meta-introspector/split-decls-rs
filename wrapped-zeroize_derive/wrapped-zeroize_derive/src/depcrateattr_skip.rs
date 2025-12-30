// Generated macro for attr_skip (function)
macro_rules! Depcrateattr_skip {
() => {
// Module: crate
// Provides: {"attr_skip"}
// Dependencies: {}
fn attr_skip (attrs : & [Attribute]) -> bool { let mut result = false ; for attr in attrs . iter () . map (| attr | & attr . meta) { if let Meta :: List (list) = attr { if list . path . is_ident (ZEROIZE_ATTR) { for meta in list . parse_args_with (Punctuated :: < Meta , Comma > :: parse_terminated) . unwrap_or_else (| e | panic ! ("error parsing attribute: {list:?} ({e})")) { if let Meta :: Path (path) = meta { if path . is_ident ("skip") { assert ! (! result , "duplicate #[zeroize] skip flags") ; result = true ; } } } } } } result }
};
}
