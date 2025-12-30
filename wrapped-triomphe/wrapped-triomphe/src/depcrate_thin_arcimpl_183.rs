// Generated macro for impl_183 (impl)
macro_rules! Depcrate_thin_arcimpl_183 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_183"}
// Dependencies: {}
impl < H , T > Drop for ThinArc < H , T > { # [inline] fn drop (& mut self) { let _ = Arc :: protected_from_thin (ThinArc { ptr : self . ptr , phantom : PhantomData , }) ; } }
};
}
