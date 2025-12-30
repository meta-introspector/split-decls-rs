// Generated macro for impl_99 (impl)
macro_rules! Depcrate_knownhostsimpl_99 {
() => {
// Module: crate::knownhosts
// Provides: {"impl_99"}
// Dependencies: {}
impl Host { # [doc = " This is `None` if no plain text host name exists."] pub fn name (& self) -> Option < & str > { self . name . as_ref () . map (String :: as_str) } # [doc = " Returns the key in base64/printable format"] pub fn key (& self) -> & str { & self . key } unsafe fn from_raw (raw : * mut raw :: libssh2_knownhost) -> Self { let name = :: opt_bytes (& raw , (* raw) . name) . and_then (| s | String :: from_utf8 (s . to_vec ()) . ok ()) ; let key = :: opt_bytes (& raw , (* raw) . key) . unwrap () ; let key = String :: from_utf8 (key . to_vec ()) . unwrap () ; Self { name , key } } }
};
}
