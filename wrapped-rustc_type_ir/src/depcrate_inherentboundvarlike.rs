// Generated macro for BoundVarLike (trait)
macro_rules! Depcrate_inherentBoundVarLike {
() => {
// Module: crate::inherent
// Provides: {"BoundVarLike"}
// Dependencies: {}
pub trait BoundVarLike < I : Interner > : Copy + Debug + Hash + Eq { fn var (self) -> ty :: BoundVar ; fn assert_eq (self , var : I :: BoundVarKind) ; }
};
}
