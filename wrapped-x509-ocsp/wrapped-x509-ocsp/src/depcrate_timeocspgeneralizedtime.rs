// Generated macro for OcspGeneralizedTime (struct)
macro_rules! Depcrate_timeOcspGeneralizedTime {
() => {
// Module: crate::time
// Provides: {"OcspGeneralizedTime"}
// Dependencies: {}
# [doc = " [`GeneralizedTime`] wrapper for easy conversion from legacy `UTCTime`"] # [doc = ""] # [doc = " OCSP does not support `UTCTime` while many other X.509 structures do."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct OcspGeneralizedTime (pub GeneralizedTime) ;
};
}
