// Generated macro for impl_29 (impl)
macro_rules! Depcrate_builderimpl_29 {
() => {
// Module: crate::builder
// Provides: {"impl_29"}
// Dependencies: {}
impl BlockFrame { fn is_tail_expr (& self) -> bool { match * self { BlockFrame :: TailExpr { .. } => true , BlockFrame :: Statement { .. } | BlockFrame :: SubExpr => false , } } fn is_statement (& self) -> bool { match * self { BlockFrame :: Statement { .. } => true , BlockFrame :: TailExpr { .. } | BlockFrame :: SubExpr => false , } } }
};
}
