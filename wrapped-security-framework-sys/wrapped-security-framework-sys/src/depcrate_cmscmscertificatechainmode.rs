// Generated macro for CMSCertificateChainMode (enum)
macro_rules! Depcrate_cmsCMSCertificateChainMode {
() => {
// Module: crate::cms
// Provides: {"CMSCertificateChainMode"}
// Dependencies: {}
# [repr (i32)] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum CMSCertificateChainMode { kCMSCertificateNone = 0 , kCMSCertificateSignerOnly = 1 , kCMSCertificateChain = 2 , kCMSCertificateChainWithRoot = 3 , kCMSCertificateChainWithRootOrFail = 4 , }
};
}
