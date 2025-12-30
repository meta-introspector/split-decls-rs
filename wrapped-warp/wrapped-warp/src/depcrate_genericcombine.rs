// Generated macro for Combine (trait)
macro_rules! Depcrate_genericCombine {
() => {
// Module: crate::generic
// Provides: {"Combine"}
// Dependencies: {}
pub trait Combine < T : HList > { type Output : HList ; fn combine (self , other : T) -> Self :: Output ; }
};
}
