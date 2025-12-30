// Generated macro for impl_266 (impl)
macro_rules! Depcrate_crl_typesimpl_266 {
() => {
// Module: crate::crl::types
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'a > From < BorrowedCertRevocationList < 'a > > for CertRevocationList < 'a > { fn from (crl : BorrowedCertRevocationList < 'a >) -> Self { Self :: Borrowed (crl) } }
};
}
