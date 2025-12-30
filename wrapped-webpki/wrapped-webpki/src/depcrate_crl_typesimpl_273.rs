// Generated macro for impl_273 (impl)
macro_rules! Depcrate_crl_typesimpl_273 {
() => {
// Module: crate::crl::types
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a BorrowedCertRevocationList < 'a > { type Item = Result < BorrowedRevokedCert < 'a > , Error > ; type IntoIter = DerIterator < 'a , BorrowedRevokedCert < 'a > > ; fn into_iter (self) -> Self :: IntoIter { DerIterator :: new (self . revoked_certs) } }
};
}
