// Generated macro for symmetric_diff (function)
macro_rules! Depcrate_xtasksymmetric_diff {
() => {
// Module: crate::xtask
// Provides: {"symmetric_diff"}
// Dependencies: {}
# [doc = " Using a \"symmetric difference\" between base and head."] fn symmetric_diff < 'a > (repo : & 'a git2 :: Repository , base : & 'a git2 :: Commit < 'a > , head : & 'a git2 :: Commit < 'a > ,) -> CargoResult < git2 :: Diff < 'a > > { let ancestor_oid = repo . merge_base (base . id () , head . id ()) ? ; let ancestor_commit = repo . find_commit (ancestor_oid) ? ; let ancestor_tree = ancestor_commit . as_object () . peel_to_tree () ? ; let head_tree = head . as_object () . peel_to_tree () ? ; let diff = repo . diff_tree_to_tree (Some (& ancestor_tree) , Some (& head_tree) , Default :: default ()) ? ; tracing :: info ! (merge_base = % ancestor_commit . id () , base = % base . id () , head = % head . id () , "git diff base...head") ; Ok (diff) }
};
}
