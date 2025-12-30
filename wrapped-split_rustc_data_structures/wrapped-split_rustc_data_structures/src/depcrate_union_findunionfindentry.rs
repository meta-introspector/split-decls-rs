// Generated macro for UnionFindEntry (struct)
macro_rules! Depcrate_union_findUnionFindEntry {
() => {
// Module: crate::union_find
// Provides: {"UnionFindEntry"}
// Dependencies: {}
# [derive (Debug)] struct UnionFindEntry < Key > { # [doc = " Transitively points towards the \"root\" of the set containing this key."] # [doc = ""] # [doc = " Invariant: A root key is its own parent."] parent : Key , # [doc = " When merging two \"root\" keys, their ranks determine which key becomes"] # [doc = " the new root, to prevent the parent tree from becoming unnecessarily"] # [doc = " tall. See [`UnionFind::unify`] for details."] rank : u32 , }
};
}
