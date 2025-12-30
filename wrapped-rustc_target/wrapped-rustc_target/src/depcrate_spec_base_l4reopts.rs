// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_l4reopts {
() => {
// Module: crate::spec::base::l4re
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "l4re" . into () , env : "uclibc" . into () , linker_flavor : LinkerFlavor :: Unix (Cc :: No) , panic_strategy : PanicStrategy :: Abort , linker : Some ("l4-bender" . into ()) , families : cvs ! ["unix"] , relocation_model : RelocModel :: Static , .. Default :: default () } }
};
}
