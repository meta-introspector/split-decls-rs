// Generated macro for impl_37 (impl)
macro_rules! Depcrate_cert_statusimpl_37 {
() => {
// Module: crate::cert_status
// Provides: {"impl_37"}
// Dependencies: {}
impl From < RevokedCert > for RevokedInfo { # [doc = " Converts [`RevokedCert`] to [`RevokedInfo`]."] # [doc = ""] # [doc = " Attempts to extract the [`CrlReason`]. If it fails, the `CrlReason` is set to `None`."] fn from (rc : RevokedCert) -> Self { Self :: from (& rc) } }
};
}
