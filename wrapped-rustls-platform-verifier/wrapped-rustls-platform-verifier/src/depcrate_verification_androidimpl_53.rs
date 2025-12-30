// Generated macro for impl_53 (impl)
macro_rules! Depcrate_verification_androidimpl_53 {
() => {
// Module: crate::verification::android
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (any (test , feature = "ffi-testing"))] impl Drop for Verifier { fn drop (& mut self) { with_context :: < _ , () > (| cx , env | { env . call_static_method (CERT_VERIFIER_CLASS . get (cx) ? , "clearMockRoots" , "()V" , & []) ? . v () ? ; Ok (()) }) . expect ("failed to clear test roots") } }
};
}
