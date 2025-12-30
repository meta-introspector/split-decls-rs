// Generated macro for filter_attrs (function)
macro_rules! Depcrate_attrfilter_attrs {
() => {
// Module: crate::attr
// Provides: {"filter_attrs"}
// Dependencies: {}
fn filter_attrs < 'a > (cx : & 'a Context , attrs : & 'a [syn :: Attribute] , pos : Position ,) -> Vec < (& 'static AttrDef , Meta) > { let mut counter = vec ! [0 ; ATTRS . len ()] ; attrs . iter () . filter (| attr | attr . path () . is_ident (ATTR_NAME)) . filter_map (move | attr | match & attr . meta { Meta :: List (list) => match list . parse_args_with (Punctuated :: < syn :: Meta , syn :: Token ! [,] > :: parse_terminated) { Ok (list) => Some (list) , Err (e) => { cx . error (e) ; None } } , m => { cx . error (format_err ! (m , "expected `#[{}(...)]`" , ATTR_NAME)) ; None } }) . flatten () . filter_map (move | m | match m . path () . get_ident () { Some (p) => match ATTRS . iter () . position (| a | p == a . name) { Some (pos) => { counter [pos] += 1 ; if counter [pos] == 1 { Some ((& ATTRS [pos] , m)) } else { cx . error (format_err ! (& m , "duplicate #[{}({})] attribute" , ATTR_NAME , p)) ; None } } None => { cx . error (format_err ! (p , "unknown {} attribute `{}`" , ATTR_NAME , p)) ; None } } , None => { cx . error (format_err ! (m , "expected identifier, found path")) ; None } }) . filter (| (def , meta) | ! def . early_check (cx , pos , meta)) . collect () }
};
}
