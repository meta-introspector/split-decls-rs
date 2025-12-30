// Generated macro for impl_352 (impl)
macro_rules! Depcrate_markerimpl_352 {
() => {
// Module: crate::marker
// Provides: {"impl_352"}
// Dependencies: {}
impl < T > FromDyn < T > { # [inline (always)] pub fn from (val : T) -> Self { assert ! (crate :: sync :: is_dyn_thread_safe ()) ; FromDyn (val) } # [inline (always)] pub fn derive < O > (& self , val : O) -> FromDyn < O > { FromDyn (val) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }
};
}
