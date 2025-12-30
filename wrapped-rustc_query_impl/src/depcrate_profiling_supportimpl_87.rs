// Generated macro for impl_87 (impl)
macro_rules! Depcrate_profiling_supportimpl_87 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : SpecIntoSelfProfilingString > IntoSelfProfilingString for T { fn to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { self . spec_to_self_profile_string (builder) } }
};
}
