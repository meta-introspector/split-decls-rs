// Generated macro for impl_103 (impl)
macro_rules! Depcrate_nameimpl_103 {
() => {
// Module: crate::name
// Provides: {"impl_103"}
// Dependencies: {}
# [doc = " Parse an [`RdnSequence`] string."] # [doc = ""] # [doc = " Follows the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl FromStr for RdnSequence { type Err = der :: Error ; fn from_str (s : & str) -> der :: Result < Self > { let mut parts = split (s , b',') . map (RelativeDistinguishedName :: from_str) . collect :: < der :: Result < Vec < _ > > > () ? ; parts . reverse () ; Ok (Self (parts)) } }
};
}
