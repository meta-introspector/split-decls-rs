// Generated macro for traverse_candidate (function)
macro_rules! Depcrate_builder_matchestraverse_candidate {
() => {
// Module: crate::builder::matches
// Provides: {"traverse_candidate"}
// Dependencies: {}
# [doc = " A depth-first traversal of the `Candidate` and all of its recursive"] # [doc = " subcandidates."] # [doc = ""] # [doc = " This signature is very generic, to support traversing candidate trees by"] # [doc = " reference or by value, and to allow a mutable \"context\" to be shared by the"] # [doc = " traversal callbacks. Most traversals can use the simpler"] # [doc = " [`Candidate::visit_leaves`] wrapper instead."] fn traverse_candidate < 'tcx , C , T , I > (candidate : C , context : & mut T , visit_leaf : & mut impl FnMut (C , & mut T) , get_children : impl Copy + Fn (C , & mut T) -> I , complete_children : impl Copy + Fn (& mut T) ,) where C : Borrow < Candidate < 'tcx > > , I : Iterator < Item = C > , { if candidate . borrow () . subcandidates . is_empty () { visit_leaf (candidate , context) } else { for child in get_children (candidate , context) { traverse_candidate (child , context , visit_leaf , get_children , complete_children) ; } complete_children (context) } }
};
}
