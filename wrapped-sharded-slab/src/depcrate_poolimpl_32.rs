// Generated macro for impl_32 (impl)
macro_rules! Depcrate_poolimpl_32 {
() => {
// Module: crate::pool
// Provides: {"impl_32"}
// Dependencies: {}
impl < T , C > OwnedRef < T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
};
}
