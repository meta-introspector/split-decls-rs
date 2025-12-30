// Generated macro for impl_338 (impl)
macro_rules! Depcrateimpl_338 {
() => {
// Module: crate
// Provides: {"impl_338"}
// Dependencies: {}
# [cfg (feature = "std")] # [allow (deprecated)] # [cfg (not (target_feature = "atomics"))] impl < T : crate :: convert :: FromWasmAbi + 'static > Deref for JsStatic < T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . __inner . with (| ptr | & * (ptr as * const T)) } } }
};
}
