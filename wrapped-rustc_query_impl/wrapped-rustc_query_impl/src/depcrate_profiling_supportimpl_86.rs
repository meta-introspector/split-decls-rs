// Generated macro for impl_86 (impl)
macro_rules! Depcrate_profiling_supportimpl_86 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : Debug > IntoSelfProfilingString for T { default fn to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ > ,) -> StringId { let s = format ! ("{self:?}") ; builder . profiler . alloc_string (& s [..]) } }
};
}
