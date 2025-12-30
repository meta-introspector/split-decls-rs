// Generated macro for list_of_nested_meta_items_from_tokens (function)
macro_rules! Depcrate_attrlist_of_nested_meta_items_from_tokens {
() => {
// Module: crate::attr
// Provides: {"list_of_nested_meta_items_from_tokens"}
// Dependencies: {}
fn list_of_nested_meta_items_from_tokens (mut tts : & [proc_macro2 :: TokenTree]) -> Option < Delimited < NestedMetaItem , tokens :: Comma > > { let mut delimited = Delimited :: new () ; let mut first = true ; while ! tts . is_empty () { let prev_comma = if first { first = false ; None } else if let TokenNode :: Op (',' , Spacing :: Alone) = tts [0] . kind { let tok = tokens :: Comma ([Span (tts [0] . span)]) ; tts = & tts [1 ..] ; if tts . is_empty () { break } Some (tok) } else { return None } ; let (nested , rest) = match nested_meta_item_from_tokens (tts) { Some (pair) => pair , None => return None , } ; match prev_comma { Some (comma) => delimited . push_next (nested , comma) , None => delimited . push_first (nested) , } tts = rest ; } Some (delimited) }
};
}
