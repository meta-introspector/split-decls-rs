// Generated macro for impl_92 (impl)
macro_rules! Depcrate_profiling_supportimpl_92 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_92"}
// Dependencies: {}
impl SpecIntoSelfProfilingString for LocalDefId { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { builder . def_id_to_string_id (DefId { krate : LOCAL_CRATE , index : self . local_def_index }) } }
};
}
