// Generated macro for impl_93 (impl)
macro_rules! Depcrate_profiling_supportimpl_93 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_93"}
// Dependencies: {}
impl < T0 , T1 > SpecIntoSelfProfilingString for (T0 , T1) where T0 : SpecIntoSelfProfilingString , T1 : SpecIntoSelfProfilingString , { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { let val0 = self . 0 . to_self_profile_string (builder) ; let val1 = self . 1 . to_self_profile_string (builder) ; let components = & [StringComponent :: Value ("(") , StringComponent :: Ref (val0) , StringComponent :: Value (",") , StringComponent :: Ref (val1) , StringComponent :: Value (")") ,] ; builder . profiler . alloc_string (components) } }
};
}
