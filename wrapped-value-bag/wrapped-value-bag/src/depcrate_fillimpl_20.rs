// Generated macro for impl_20 (impl)
macro_rules! Depcrate_fillimpl_20 {
() => {
// Module: crate::fill
// Provides: {"impl_20"}
// Dependencies: {}
impl < F > Fill for F where F : Fn (Slot) -> Result < () , Error > , { fn fill (& self , slot : Slot) -> Result < () , Error > { (self) (slot) } }
};
}
