// Generated macro for impl_1167 (impl)
macro_rules! Depcrate_patternsimpl_1167 {
() => {
// Module: crate::patterns
// Provides: {"impl_1167"}
// Dependencies: {}
impl < 'a , T : Rewrite > Rewrite for RangeOperand < 'a , T > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match & self . operand { None => Ok ("" . to_owned ()) , Some (ref exp) => exp . rewrite_result (context , shape) , } } }
};
}
