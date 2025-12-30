// Generated macro for impl_111 (impl)
macro_rules! Depcrate_nameimpl_111 {
() => {
// Module: crate::name
// Provides: {"impl_111"}
// Dependencies: {}
# [doc = " Parse a [`RelativeDistinguishedName`] string."] # [doc = ""] # [doc = " This function follows the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl FromStr for RelativeDistinguishedName { type Err = der :: Error ; fn from_str (s : & str) -> der :: Result < Self > { split (s , b'+') . map (AttributeTypeAndValue :: from_str) . collect :: < der :: Result < Vec < _ > > > () ? . try_into () . map (Self) } }
};
}
