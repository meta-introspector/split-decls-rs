// Generated macro for merge_use_trees_inner (function)
macro_rules! Depcrate_importsmerge_use_trees_inner {
() => {
// Module: crate::imports
// Provides: {"merge_use_trees_inner"}
// Dependencies: {}
fn merge_use_trees_inner (trees : & mut Vec < UseTree > , use_tree : UseTree , merge_by : SharedPrefix) { struct SimilarTree < 'a > { similarity : usize , path_len : usize , tree : & 'a mut UseTree , } let similar_trees = trees . iter_mut () . filter_map (| tree | { if tree . share_prefix (& use_tree , merge_by) { let similarity = if merge_by == SharedPrefix :: One { tree . path . iter () . zip (& use_tree . path) . take_while (| (a , b) | a . equal_except_alias (b)) . count () } else { 0 } ; let path_len = tree . path . len () ; Some (SimilarTree { similarity , tree , path_len , }) } else { None } }) ; if use_tree . path . len () == 1 && merge_by == SharedPrefix :: Crate { if let Some (tree) = similar_trees . min_by_key (| tree | tree . path_len) { if tree . path_len == 1 { return ; } } } else if merge_by == SharedPrefix :: One { if let Some (sim_tree) = similar_trees . max_by_key (| tree | tree . similarity) { if sim_tree . similarity > 0 { sim_tree . tree . merge (& use_tree , merge_by) ; return ; } } } else if let Some (sim_tree) = similar_trees . max_by_key (| tree | tree . path_len) { if sim_tree . path_len > 1 { sim_tree . tree . merge (& use_tree , merge_by) ; return ; } } trees . push (use_tree) ; trees . sort () ; trees . dedup () ; }
};
}
