// Generated macro for impl_269 (impl)
macro_rules! Depcrate_crl_typesimpl_269 {
() => {
// Module: crate::crl::types
// Provides: {"impl_269"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl OwnedCertRevocationList { # [doc = " Try to parse the given bytes as a RFC 5280[^1] profile Certificate Revocation List (CRL)."] # [doc = ""] # [doc = " Webpki does not support:"] # [doc = "   * CRL versions other than version 2."] # [doc = "   * CRLs missing the next update field."] # [doc = "   * CRLs missing certificate revocation list extensions."] # [doc = "   * Delta CRLs."] # [doc = "   * CRLs larger than (2^32)-1 bytes in size."] # [doc = ""] # [doc = " See [BorrowedCertRevocationList::from_der] for more details."] # [doc = ""] # [doc = " [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>"] pub fn from_der (crl_der : & [u8]) -> Result < Self , Error > { BorrowedCertRevocationList :: from_der (crl_der) ? . to_owned () } fn find_serial (& self , serial : & [u8]) -> Result < Option < BorrowedRevokedCert < '_ > > , Error > { Ok (self . revoked_certs . get (serial) . map (| owned_revoked_cert | owned_revoked_cert . borrow ())) } }
};
}
