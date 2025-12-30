// Generated macro for impl_554 (impl)
macro_rules! Depcrate_exprimpl_554 {
() => {
// Module: crate::expr
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'ast > RhsAssignKind < 'ast > { # [allow (dead_code)] fn is_chain (& self) -> bool { match self { RhsAssignKind :: Expr (kind , _) => { matches ! (kind , ast :: ExprKind :: Try (..) | ast :: ExprKind :: Field (..) | ast :: ExprKind :: MethodCall (..) | ast :: ExprKind :: Await (_ , _)) } _ => false , } } }
};
}
