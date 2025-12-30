// Generated macro for impl_36 (impl)
macro_rules! Depcrate_cert_statusimpl_36 {
() => {
// Module: crate::cert_status
// Provides: {"impl_36"}
// Dependencies: {}
impl From < & RevokedCert > for RevokedInfo { # [doc = " Converts [`RevokedCert`] to [`RevokedInfo`]."] # [doc = ""] # [doc = " Attempts to extract the [`CrlReason`]. If it fails, the `CrlReason` is set to `None`."] fn from (rc : & RevokedCert) -> Self { Self { revocation_time : rc . revocation_date . into () , revocation_reason : match & rc . crl_entry_extensions { Some (extns) => { let mut filter = extns . iter () . filter (| extn | extn . extn_id == CrlReason :: OID) ; match filter . next () { Some (extn) => CrlReason :: from_der (extn . extn_value . as_bytes ()) . ok () , None => None , } } None => None , } , } } }
};
}
