// Generated macro for Clauses (trait)
macro_rules! Depcrate_inherentClauses {
() => {
// Module: crate::inherent
// Provides: {"Clauses"}
// Dependencies: {}
pub trait Clauses < I : Interner < Clauses = Self > > : Copy + Debug + Hash + Eq + TypeSuperVisitable < I > + TypeSuperFoldable < I > + Flags + SliceLike < Item = I :: Clause > { }
};
}
