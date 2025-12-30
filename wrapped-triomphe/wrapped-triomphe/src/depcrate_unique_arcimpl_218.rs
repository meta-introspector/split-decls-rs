// Generated macro for impl_218 (impl)
macro_rules! Depcrate_unique_arcimpl_218 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_218"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for UniqueArc < T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut (* self . 0 . ptr ()) . data } } }
};
}
