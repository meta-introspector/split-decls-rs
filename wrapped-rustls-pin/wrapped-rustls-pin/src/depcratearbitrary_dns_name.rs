// Generated macro for arbitrary_dns_name (function)
macro_rules! Depcratearbitrary_dns_name {
() => {
// Module: crate
// Provides: {"arbitrary_dns_name"}
// Dependencies: {}
# [doc = " An arbitrary `DNSName` struct, for passing to [`rustls::ClientSession::new`]."] # [doc = " `PinnedServerCertVerifier` receives the value and ignores it."] # [must_use] # [allow (clippy :: missing_panics_doc)] pub fn arbitrary_dns_name () -> webpki :: DNSName { webpki :: DNSNameRef :: try_from_ascii_str ("arbitrary1") . unwrap () . to_owned () }
};
}
