// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_solarisopts {
() => {
// Module: crate::spec::base::solaris
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "solaris" . into () , dynamic_linking : true , has_rpath : true , families : cvs ! ["unix"] , is_like_solaris : true , linker_flavor : LinkerFlavor :: Unix (Cc :: Yes) , limit_rdylib_exports : false , eh_frame_header : false , .. Default :: default () } }
};
}
