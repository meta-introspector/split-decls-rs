// Generated macro for ExprConst (trait)
macro_rules! Depcrate_inherentExprConst {
() => {
// Module: crate::inherent
// Provides: {"ExprConst"}
// Dependencies: {}
pub trait ExprConst < I : Interner < ExprConst = Self > > : Copy + Debug + Hash + Eq + Relate < I > { fn args (self) -> I :: GenericArgs ; }
};
}
