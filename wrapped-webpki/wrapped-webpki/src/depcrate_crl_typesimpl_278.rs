// Generated macro for impl_278 (impl)
macro_rules! Depcrate_crl_typesimpl_278 {
() => {
// Module: crate::crl::types
// Provides: {"impl_278"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl OwnedRevokedCert { # [doc = " Convert the owned representation of this revoked cert to a borrowed version."] pub fn borrow (& self) -> BorrowedRevokedCert < '_ > { BorrowedRevokedCert { serial_number : & self . serial_number , revocation_date : self . revocation_date , reason_code : self . reason_code , invalidity_date : self . invalidity_date , } } }
};
}
