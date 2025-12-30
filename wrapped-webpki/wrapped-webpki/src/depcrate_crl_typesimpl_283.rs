// Generated macro for impl_283 (impl)
macro_rules! Depcrate_crl_typesimpl_283 {
() => {
// Module: crate::crl::types
// Provides: {"impl_283"}
// Dependencies: {}
impl RevocationReason { # [doc = " Return an iterator over all possible [RevocationReason] variants."] pub fn iter () -> impl Iterator < Item = Self > { use RevocationReason :: * ; [Unspecified , KeyCompromise , CaCompromise , AffiliationChanged , Superseded , CessationOfOperation , CertificateHold , RemoveFromCrl , PrivilegeWithdrawn , AaCompromise ,] . into_iter () } }
};
}
