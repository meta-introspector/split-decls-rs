// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_haikuopts {
() => {
// Module: crate::spec::base::haiku
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "haiku" . into () , dynamic_linking : true , families : cvs ! ["unix"] , relro_level : RelroLevel :: Full , .. Default :: default () } }
};
}
