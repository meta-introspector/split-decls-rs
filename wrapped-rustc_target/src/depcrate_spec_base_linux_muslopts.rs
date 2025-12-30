// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_linux_muslopts {
() => {
// Module: crate::spec::base::linux_musl
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { env : "musl" . into () , pre_link_objects_self_contained : crt_objects :: pre_musl_self_contained () , post_link_objects_self_contained : crt_objects :: post_musl_self_contained () , link_self_contained : LinkSelfContainedDefault :: InferredForMusl , .. base :: linux :: opts () } }
};
}
