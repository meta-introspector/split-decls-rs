// Generated macro for impl_113 (impl)
macro_rules! Depcrate_middleware_redirectimpl_113 {
() => {
// Module: crate::middleware::redirect
// Provides: {"impl_113"}
// Dependencies: {}
impl Default for Redirect { # [doc = " Create a new instance of the Redirect middleware, which attempts to follow up to"] # [doc = " 3 redirects (not including the actual request)."] fn default () -> Self { Self { attempts : 3 } } }
};
}
