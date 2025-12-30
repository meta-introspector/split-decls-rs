// Generated macro for impl_32 (impl)
macro_rules! Depcrate_cert_statusimpl_32 {
() => {
// Module: crate::cert_status
// Provides: {"impl_32"}
// Dependencies: {}
impl CertStatus { # [doc = " Returns `CertStatus` set to `good` as defined in [RFC 6960 Section 4.2.1]"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] pub fn good () -> Self { Self :: Good (Null) } # [doc = " Returns `CertStatus` set to `revoked` as defined in [RFC 6960 Section 4.2.1]"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] pub fn revoked (info : impl Into < RevokedInfo >) -> Self { Self :: Revoked (info . into ()) } # [doc = " Returns `CertStatus` set to `unknown` as defined in [RFC 6960 Section 4.2.1]"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] pub fn unknown () -> Self { Self :: Unknown (Null) } }
};
}
