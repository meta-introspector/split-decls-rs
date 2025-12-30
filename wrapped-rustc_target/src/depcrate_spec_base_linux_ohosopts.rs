// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_linux_ohosopts {
() => {
// Module: crate::spec::base::linux_ohos
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { env : "ohos" . into () , crt_static_default : false , tls_model : TlsModel :: Emulated , has_thread_local : false , .. base :: linux :: opts () } }
};
}
