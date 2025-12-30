// Generated macro for normalize_use_trees_with_granularity (function)
macro_rules! Depcrate_importsnormalize_use_trees_with_granularity {
() => {
// Module: crate::imports
// Provides: {"normalize_use_trees_with_granularity"}
// Dependencies: {}
pub (crate) fn normalize_use_trees_with_granularity (use_trees : Vec < UseTree > , import_granularity : ImportGranularity ,) -> Vec < UseTree > { let merge_by = match import_granularity { ImportGranularity :: Item => return flatten_use_trees (use_trees , ImportGranularity :: Item) , ImportGranularity :: Preserve => return use_trees , ImportGranularity :: Crate => SharedPrefix :: Crate , ImportGranularity :: Module => SharedPrefix :: Module , ImportGranularity :: One => SharedPrefix :: One , } ; let mut result = Vec :: with_capacity (use_trees . len ()) ; for use_tree in use_trees { if use_tree . contains_comment () || use_tree . attrs . is_some () { result . push (use_tree) ; continue ; } for mut flattened in use_tree . flatten (import_granularity) { if let Some (tree) = result . iter_mut () . find (| tree | tree . share_prefix (& flattened , merge_by)) { tree . merge (& flattened , merge_by) ; } else { if merge_by == SharedPrefix :: Module { flattened = flattened . nest_trailing_self () ; } result . push (flattened) ; } } } result }
};
}
