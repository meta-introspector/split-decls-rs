// Generated macro for impl_89 (impl)
macro_rules! Depcrate_profiling_supportimpl_89 {
() => {
// Module: crate::profiling_support
// Provides: {"impl_89"}
// Dependencies: {}
impl SpecIntoSelfProfilingString for DefId { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { builder . def_id_to_string_id (* self) } }
};
}
