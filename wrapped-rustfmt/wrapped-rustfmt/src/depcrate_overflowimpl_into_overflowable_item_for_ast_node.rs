// Generated macro for impl_into_overflowable_item_for_ast_node (macro)
macro_rules! Depcrate_overflowimpl_into_overflowable_item_for_ast_node {
() => {
// Module: crate::overflow
// Provides: {"impl_into_overflowable_item_for_ast_node"}
// Dependencies: {}
macro_rules ! impl_into_overflowable_item_for_ast_node { ($ ($ ast_node : ident) ,*) => { $ (impl <'a > IntoOverflowableItem <'a > for ast ::$ ast_node { fn into_overflowable_item (&'a self) -> OverflowableItem <'a > { OverflowableItem ::$ ast_node (self) } }) * } }
};
}
