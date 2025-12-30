// Generated macro for impl_420 (impl)
macro_rules! Depcrate_obligation_forestimpl_420 {
() => {
// Module: crate::obligation_forest
// Provides: {"impl_420"}
// Dependencies: {}
impl < O > Node < O > { fn new (parent : Option < usize > , obligation : O , obligation_tree_id : ObligationTreeId) -> Node < O > { Node { obligation , state : Cell :: new (NodeState :: Pending) , dependents : if let Some (parent_index) = parent { vec ! [parent_index] } else { vec ! [] } , has_parent : parent . is_some () , obligation_tree_id , } } }
};
}
