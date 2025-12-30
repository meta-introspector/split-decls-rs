// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_unikraft_linux_muslopts {
() => {
// Module: crate::spec::base::unikraft_linux_musl
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "linux" . into () , env : "musl" . into () , vendor : "unikraft" . into () , linker : Some ("kraftld" . into ()) , relocation_model : RelocModel :: Static , families : cvs ! ["unix"] , has_thread_local : true , panic_strategy : PanicStrategy :: Abort , .. Default :: default () } }
};
}
