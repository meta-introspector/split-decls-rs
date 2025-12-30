// Generated macro for compare_items (function)
macro_rules! Depcrate_reordercompare_items {
() => {
// Module: crate::reorder
// Provides: {"compare_items"}
// Dependencies: {}
# [doc = " Choose the ordering between the given two items."] fn compare_items (a : & ast :: Item , b : & ast :: Item , context : & RewriteContext < '_ >) -> Ordering { let style_edition = context . config . style_edition () ; match (& a . kind , & b . kind) { (& ast :: ItemKind :: Mod (_ , a_ident , _) , & ast :: ItemKind :: Mod (_ , b_ident , _)) => { if style_edition <= StyleEdition :: Edition2021 { a_ident . as_str () . cmp (b_ident . as_str ()) } else { version_sort (a_ident . as_str () , b_ident . as_str ()) } } (& ast :: ItemKind :: ExternCrate (ref a_name , a_ident) , & ast :: ItemKind :: ExternCrate (ref b_name , b_ident) ,) => { let a_orig_name = a_name . unwrap_or (a_ident . name) ; let b_orig_name = b_name . unwrap_or (b_ident . name) ; let result = if style_edition <= StyleEdition :: Edition2021 { a_orig_name . as_str () . cmp (b_orig_name . as_str ()) } else { version_sort (a_orig_name . as_str () , b_orig_name . as_str ()) } ; if result != Ordering :: Equal { return result ; } match (a_name , b_name) { (Some (..) , None) => Ordering :: Greater , (None , Some (..)) => Ordering :: Less , (None , None) => Ordering :: Equal , (Some (..) , Some (..)) if style_edition <= StyleEdition :: Edition2021 => { a_ident . as_str () . cmp (b_ident . as_str ()) } (Some (..) , Some (..)) => version_sort (a_ident . as_str () , b_ident . as_str ()) , } } _ => unreachable ! () , } }
};
}
