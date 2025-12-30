// Generated macro for rewrite_attrs (function)
macro_rules! Depcrate_itemsrewrite_attrs {
() => {
// Module: crate::items
// Provides: {"rewrite_attrs"}
// Dependencies: {}
# [doc = " Rewrite the attributes of an item."] fn rewrite_attrs (context : & RewriteContext < '_ > , item : & ast :: Item , item_str : & str , shape : Shape ,) -> RewriteResult { let attrs = filter_inline_attrs (& item . attrs , item . span ()) ; let attrs_str = attrs . rewrite_result (context , shape) ? ; let missed_span = if attrs . is_empty () { mk_sp (item . span . lo () , item . span . lo ()) } else { mk_sp (attrs [attrs . len () - 1] . span . hi () , item . span . lo ()) } ; let allow_extend = if attrs . len () == 1 { let line_len = attrs_str . len () + 1 + item_str . len () ; ! attrs . first () . unwrap () . is_doc_comment () && context . config . inline_attribute_width () >= line_len } else { false } ; combine_strs_with_missing_comments (context , & attrs_str , item_str , missed_span , shape , allow_extend ,) }
};
}
