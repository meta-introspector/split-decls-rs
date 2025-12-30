// Generated macro for Tys (trait)
macro_rules! Depcrate_inherentTys {
() => {
// Module: crate::inherent
// Provides: {"Tys"}
// Dependencies: {}
pub trait Tys < I : Interner < Tys = Self > > : Copy + Debug + Hash + Eq + SliceLike < Item = I :: Ty > + TypeFoldable < I > + Default { fn inputs (self) -> I :: FnInputTys ; fn output (self) -> I :: Ty ; }
};
}
