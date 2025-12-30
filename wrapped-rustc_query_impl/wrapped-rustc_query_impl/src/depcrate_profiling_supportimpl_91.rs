// Generated macro for impl_91 (impl)
macro_rules! Depcrate_profiling_supportimpl_91 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_91"}
// Dependencies: {}
impl SpecIntoSelfProfilingString for DefIndex { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { builder . def_id_to_string_id (DefId { krate : LOCAL_CRATE , index : * self }) } }
};
}
