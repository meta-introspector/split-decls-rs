// Generated macro for impl_21 (impl)
macro_rules! Depcrate_poolimpl_21 {
() => {
// Module: crate::pool
// Provides: {"impl_21"}
// Dependencies: {}
impl < T , C > Ref < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
};
}
