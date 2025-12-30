// Generated macro for impl_340 (impl)
macro_rules! Depcrateimpl_340 {
() => {
// Module: crate
// Provides: {"impl_340"}
// Dependencies: {}
impl < T > JsThreadLocal < T > { pub fn with < F , R > (& 'static self , f : F) -> R where F : FnOnce (& T) -> R , { # [cfg (not (target_feature = "atomics"))] return f (self . __inner) ; # [cfg (target_feature = "atomics")] f (unsafe { & * (self . __inner) () }) } }
};
}
