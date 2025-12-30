// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_hurdopts {
() => {
// Module: crate::spec::base::hurd
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "hurd" . into () , dynamic_linking : true , families : cvs ! ["unix"] , has_rpath : true , position_independent_executables : true , relro_level : RelroLevel :: Full , has_thread_local : true , crt_static_respected : true , .. Default :: default () } }
};
}
