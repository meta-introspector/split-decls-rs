// Generated macro for impl_165 (impl)
macro_rules! Depcrate_chainsimpl_165 {
() => {
// Module: crate::chains
// Provides: {"impl_165"}
// Dependencies: {}
impl Rewrite for Chain { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { debug ! ("rewrite chain {:?} {:?}" , self , shape) ; let mut formatter = match context . config . indent_style () { IndentStyle :: Block => { Box :: new (ChainFormatterBlock :: new (self)) as Box < dyn ChainFormatter > } IndentStyle :: Visual => { Box :: new (ChainFormatterVisual :: new (self)) as Box < dyn ChainFormatter > } } ; formatter . format_root (& self . parent , context , shape) ? ; if let Some (result) = formatter . pure_root () { return wrap_str (result , context . config . max_width () , shape) . max_width_error (shape . width , self . parent . span) ; } let first = self . children . first () . unwrap_or (& self . parent) ; let last = self . children . last () . unwrap_or (& self . parent) ; let children_span = mk_sp (first . span . lo () , last . span . hi ()) ; let full_span = self . parent . span . with_hi (children_span . hi ()) ; let child_shape = formatter . child_shape (context , shape , children_span) ? ; formatter . format_children (context , child_shape) ? ; formatter . format_last_child (context , shape , child_shape) ? ; let result = formatter . join_rewrites (context , child_shape) ? ; wrap_str (result , context . config . max_width () , shape) . max_width_error (shape . width , full_span) } }
};
}
