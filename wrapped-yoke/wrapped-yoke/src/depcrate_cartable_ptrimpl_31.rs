// Generated macro for impl_31 (impl)
macro_rules! Depcrate_cartable_ptrimpl_31 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_31"}
// Dependencies: {}
impl < C > Clone for CartableOptionPointer < C > where C : CloneableCartablePointerLike , { # [inline] fn clone (& self) -> Self { let ptr = self . inner ; if ptr != sentinel_for :: < C :: Raw > () { unsafe { C :: addref_raw (ptr) } } Self { inner : self . inner , _cartable : PhantomData , } } }
};
}
