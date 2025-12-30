// Generated macro for Tree (enum)
macro_rules! Depcrate_layout_treeTree {
() => {
// Module: crate::layout::tree
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " A tree-based representation of a type layout."] # [doc = ""] # [doc = " Invariants:"] # [doc = " 1. All paths through the layout have the same length (in bytes)."] # [doc = ""] # [doc = " Nice-to-haves:"] # [doc = " 1. An `Alt` is never directly nested beneath another `Alt`."] # [doc = " 2. A `Seq` is never directly nested beneath another `Seq`."] # [doc = " 3. `Seq`s and `Alt`s with a single member do not exist."] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub (crate) enum Tree < D , R , T > where D : Def , R : Region , T : Type , { # [doc = " A sequence of successive layouts."] Seq (Vec < Self >) , # [doc = " A choice between alternative layouts."] Alt (Vec < Self >) , # [doc = " A definition node."] Def (D) , # [doc = " A reference node."] Ref (Reference < R , T >) , # [doc = " A byte node."] Byte (Byte) , }
};
}
