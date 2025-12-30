// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_androidopts {
() => {
// Module: crate::spec::base::android
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let mut base = base :: linux :: opts () ; base . os = "android" . into () ; base . is_like_android = true ; base . default_dwarf_version = 2 ; base . tls_model = TlsModel :: Emulated ; base . has_thread_local = false ; base . supported_sanitizers = SanitizerSet :: ADDRESS ; base . default_uwtable = true ; base . crt_static_respected = true ; base }
};
}
