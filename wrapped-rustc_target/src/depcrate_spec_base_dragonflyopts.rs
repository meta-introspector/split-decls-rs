// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_dragonflyopts {
() => {
// Module: crate::spec::base::dragonfly
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "dragonfly" . into () , dynamic_linking : true , families : cvs ! ["unix"] , has_rpath : true , position_independent_executables : true , relro_level : RelroLevel :: Full , has_thread_local : true , default_dwarf_version : 2 , .. Default :: default () } }
};
}
