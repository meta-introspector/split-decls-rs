// Generated macro for TrustBits (enum)
macro_rules! DepcrateTrustBits {
() => {
// Module: crate
// Provides: {"TrustBits"}
// Dependencies: {}
# [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] # [non_exhaustive] # [doc = " TrustBits describe the possible Mozilla root certificate trust bits."] pub enum TrustBits { # [doc = " certificate is trusted for Websites (e.g. TLS)."] Websites , # [doc = " certificate is trusted for Email (e.g. S/MIME)."] Email , # [doc = " certificate is trusted for code signing"] Code , # [doc = " certificate is not trusted for anything"] AllTrustBitsTurnedOff , }
};
}
