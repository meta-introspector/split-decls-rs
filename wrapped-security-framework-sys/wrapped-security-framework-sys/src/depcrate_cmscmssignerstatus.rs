// Generated macro for CMSSignerStatus (enum)
macro_rules! Depcrate_cmsCMSSignerStatus {
() => {
// Module: crate::cms
// Provides: {"CMSSignerStatus"}
// Dependencies: {}
# [repr (i32)] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum CMSSignerStatus { kCMSSignerUnsigned = 0 , kCMSSignerValid = 1 , kCMSSignerNeedsDetachedContent = 2 , kCMSSignerInvalidSignature = 3 , kCMSSignerInvalidCert = 4 , kCMSSignerInvalidIndex = 5 , }
};
}
