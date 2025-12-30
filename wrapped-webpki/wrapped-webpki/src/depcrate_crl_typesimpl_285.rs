// Generated macro for impl_285 (impl)
macro_rules! Depcrate_crl_typesimpl_285 {
() => {
// Module: crate::crl::types
// Provides: {"impl_285"}
// Dependencies: {}
impl TryFrom < u8 > for RevocationReason { type Error = Error ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { 0 => Ok (Self :: Unspecified) , 1 => Ok (Self :: KeyCompromise) , 2 => Ok (Self :: CaCompromise) , 3 => Ok (Self :: AffiliationChanged) , 4 => Ok (Self :: Superseded) , 5 => Ok (Self :: CessationOfOperation) , 6 => Ok (Self :: CertificateHold) , 8 => Ok (Self :: RemoveFromCrl) , 9 => Ok (Self :: PrivilegeWithdrawn) , 10 => Ok (Self :: AaCompromise) , _ => Err (Error :: UnsupportedRevocationReason) , } } }
};
}
