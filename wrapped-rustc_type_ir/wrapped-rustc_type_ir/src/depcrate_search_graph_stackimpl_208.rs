// Generated macro for impl_208 (impl)
macro_rules! Depcrate_search_graph_stackimpl_208 {
() => {
// Module: crate::search_graph::stack
// Provides: {"impl_208"}
// Dependencies: {}
impl < X : Cx > Index < StackDepth > for Stack < X > { type Output = StackEntry < X > ; fn index (& self , index : StackDepth) -> & StackEntry < X > { & self . entries [index] } }
};
}
