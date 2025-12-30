// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_redoxopts {
() => {
// Module: crate::spec::base::redox
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "redox" . into () , env : "relibc" . into () , dynamic_linking : true , families : cvs ! ["unix"] , has_rpath : true , position_independent_executables : true , relro_level : RelroLevel :: Full , has_thread_local : true , crt_static_default : true , crt_static_respected : true , crt_static_allows_dylibs : true , late_link_args : TargetOptions :: link_args (LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , & ["-lgcc"]) , .. Default :: default () } }
};
}
