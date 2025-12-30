// Generated macro for rewrite_extern_crate (function)
macro_rules! Depcrate_itemsrewrite_extern_crate {
() => {
// Module: crate::items
// Provides: {"rewrite_extern_crate"}
// Dependencies: {}
# [doc = " Rewrite `extern crate foo;`."] # [doc = " The given shape is used to format the extern crate's attributes."] pub (crate) fn rewrite_extern_crate (context : & RewriteContext < '_ > , item : & ast :: Item , attrs_shape : Shape ,) -> RewriteResult { assert ! (is_extern_crate (item)) ; let new_str = context . snippet (item . span) ; let item_str = if contains_comment (new_str) { new_str . to_owned () } else { let no_whitespace = & new_str . split_whitespace () . collect :: < Vec < & str > > () . join (" ") ; String :: from (& * Regex :: new (r"\s;") . unwrap () . replace (no_whitespace , ";")) } ; rewrite_attrs (context , item , & item_str , attrs_shape) }
};
}
