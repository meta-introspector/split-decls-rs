// Generated macro for impl_106 (impl)
macro_rules! Depcrateimpl_106 {
() => {
// Module: crate
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg (feature = "gecko-ffi")] impl < T , const N : usize > Deref for AutoThinVec < T , N > { type Target = ThinVec < T > ; fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
