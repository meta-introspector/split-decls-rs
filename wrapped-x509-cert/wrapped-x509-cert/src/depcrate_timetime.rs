// Generated macro for Time (enum)
macro_rules! Depcrate_timeTime {
() => {
// Module: crate::time
// Provides: {"Time"}
// Dependencies: {}
# [doc = " X.501 `Time` as defined in [RFC 5280 Section 4.1.2.5]."] # [doc = ""] # [doc = " Schema definition from [RFC 5280 Appendix A]:"] # [doc = ""] # [doc = " ```text"] # [doc = " Time ::= CHOICE {"] # [doc = "      utcTime        UTCTime,"] # [doc = "      generalTime    GeneralizedTime"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.5]: https://tools.ietf.org/html/rfc5280#section-4.1.2.5"] # [doc = " [RFC 5280 Appendix A]: https://tools.ietf.org/html/rfc5280#page-117"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Choice , Copy , Clone , Debug , Eq , PartialEq , ValueOrd)] pub enum Time { # [doc = " Legacy UTC time (has 2-digit year, valid from 1970 to 2049)."] # [doc = ""] # [doc = " Note: RFC 5280 specifies 1950-2049, however due to common operations working on"] # [doc = " `UNIX_EPOCH` this implementation's lower bound is 1970."] # [asn1 (type = "UTCTime")] UtcTime (UtcTime) , # [doc = " Modern [`GeneralizedTime`] encoding with 4-digit year."] # [asn1 (type = "GeneralizedTime")] GeneralTime (GeneralizedTime) , }
};
}
