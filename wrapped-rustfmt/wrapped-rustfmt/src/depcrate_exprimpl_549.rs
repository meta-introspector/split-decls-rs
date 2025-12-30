// Generated macro for impl_549 (impl)
macro_rules! Depcrate_exprimpl_549 {
() => {
// Module: crate::expr
// Provides: {"impl_549"}
// Dependencies: {}
impl < 'ast > RhsAssignKind < 'ast > { # [allow (dead_code)] fn is_chain (& self) -> bool { match self { RhsAssignKind :: Expr (kind , _) => { matches ! (kind , ast :: ExprKind :: Try (..) | ast :: ExprKind :: Field (..) | ast :: ExprKind :: MethodCall (..) | ast :: ExprKind :: Await (_ , _)) } _ => false , } } }
};
}
