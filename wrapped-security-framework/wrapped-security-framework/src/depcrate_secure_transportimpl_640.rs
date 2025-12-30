// Generated macro for impl_640 (impl)
macro_rules! Depcrate_secure_transportimpl_640 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_640"}
// Dependencies: {}
impl SslAuthenticate { # [doc = " Require a client certificate."] pub const ALWAYS : Self = Self (kAlwaysAuthenticate) ; # [doc = " Do not request a client certificate."] pub const NEVER : Self = Self (kNeverAuthenticate) ; # [doc = " Request but do not require a client certificate."] pub const TRY : Self = Self (kTryAuthenticate) ; }
};
}
