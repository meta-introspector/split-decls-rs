// Generated macro for RevocationCheckDepth (enum)
macro_rules! Depcrate_crlRevocationCheckDepth {
() => {
// Module: crate::crl
// Provides: {"RevocationCheckDepth"}
// Dependencies: {}
# [doc = " Describes how much of a certificate chain is checked for revocation status."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum RevocationCheckDepth { # [doc = " Only check the end entity (leaf) certificate's revocation status."] EndEntity , # [doc = " Check the revocation status of the end entity (leaf) and all intermediates."] Chain , }
};
}
