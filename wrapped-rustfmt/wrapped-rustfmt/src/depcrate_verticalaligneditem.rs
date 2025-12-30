// Generated macro for AlignedItem (trait)
macro_rules! Depcrate_verticalAlignedItem {
() => {
// Module: crate::vertical
// Provides: {"AlignedItem"}
// Dependencies: {}
pub (crate) trait AlignedItem { fn skip (& self) -> bool ; fn get_span (& self) -> Span ; fn rewrite_prefix (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult ; fn rewrite_aligned_item (& self , context : & RewriteContext < '_ > , shape : Shape , prefix_max_width : usize ,) -> RewriteResult ; }
};
}
