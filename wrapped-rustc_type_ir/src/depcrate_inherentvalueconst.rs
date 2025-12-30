// Generated macro for ValueConst (trait)
macro_rules! Depcrate_inherentValueConst {
() => {
// Module: crate::inherent
// Provides: {"ValueConst"}
// Dependencies: {}
pub trait ValueConst < I : Interner < ValueConst = Self > > : Copy + Debug + Hash + Eq { fn ty (self) -> I :: Ty ; fn valtree (self) -> I :: ValTree ; }
};
}
