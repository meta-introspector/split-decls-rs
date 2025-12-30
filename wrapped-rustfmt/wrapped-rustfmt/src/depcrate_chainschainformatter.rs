// Generated macro for ChainFormatter (trait)
macro_rules! Depcrate_chainsChainFormatter {
() => {
// Module: crate::chains
// Provides: {"ChainFormatter"}
// Dependencies: {}
trait ChainFormatter { fn format_root (& mut self , parent : & ChainItem , context : & RewriteContext < '_ > , shape : Shape ,) -> Result < () , RewriteError > ; fn child_shape (& self , context : & RewriteContext < '_ > , shape : Shape , span : Span ,) -> Result < Shape , ExceedsMaxWidthError > ; fn format_children (& mut self , context : & RewriteContext < '_ > , child_shape : Shape ,) -> Result < () , RewriteError > ; fn format_last_child (& mut self , context : & RewriteContext < '_ > , shape : Shape , child_shape : Shape ,) -> Result < () , RewriteError > ; fn join_rewrites (& self , context : & RewriteContext < '_ > , child_shape : Shape) -> RewriteResult ; fn pure_root (& mut self) -> Option < String > ; }
};
}
