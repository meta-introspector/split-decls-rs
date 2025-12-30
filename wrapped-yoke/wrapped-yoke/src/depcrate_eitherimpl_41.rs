// Generated macro for impl_41 (impl)
macro_rules! Depcrate_eitherimpl_41 {
() => {
// Module: crate::either
// Provides: {"impl_41"}
// Dependencies: {}
impl < C0 , C1 , T > Deref for EitherCart < C0 , C1 > where C0 : Deref < Target = T > , C1 : Deref < Target = T > , T : ? Sized , { type Target = T ; fn deref (& self) -> & T { use EitherCart :: * ; match self { A (a) => a . deref () , B (b) => b . deref () , } } }
};
}
