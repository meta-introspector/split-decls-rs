// Generated macro for CertRevocationList (enum)
macro_rules! Depcrate_crl_typesCertRevocationList {
() => {
// Module: crate::crl::types
// Provides: {"CertRevocationList"}
// Dependencies: {}
# [doc = " A RFC 5280[^1] profile Certificate Revocation List (CRL)."] # [doc = ""] # [doc = " May be either an owned, or a borrowed representation."] # [doc = ""] # [doc = " [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>"] # [derive (Debug , Hash)] pub enum CertRevocationList < 'a > { # [doc = " An owned representation of a CRL."] # [cfg (feature = "alloc")] Owned (OwnedCertRevocationList) , # [doc = " A borrowed representation of a CRL."] Borrowed (BorrowedCertRevocationList < 'a >) , }
};
}
