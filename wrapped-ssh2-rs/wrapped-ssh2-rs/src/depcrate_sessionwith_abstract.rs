// Generated macro for with_abstract (function)
macro_rules! Depcrate_sessionwith_abstract {
() => {
// Module: crate::session
// Provides: {"with_abstract"}
// Dependencies: {}
# [doc = " This is a little helper function that is perhaps slightly overkill for the"] # [doc = " current needs."] # [doc = " It saves the current sess->abstract pointer and replaces it with a"] # [doc = " different values for the duration of the call to the supplied lambda."] # [doc = " When the lambda returns, the original abstract value is restored"] # [doc = " and the result of the lambda is returned."] unsafe fn with_abstract < R , F : FnOnce () -> R > (sess : * mut raw :: LIBSSH2_SESSION , new_value : * mut c_void , f : F ,) -> R { let abstrakt = raw :: libssh2_session_abstract (sess) ; let old_value = * abstrakt ; * abstrakt = new_value ; let res = f () ; * abstrakt = old_value ; res }
};
}
