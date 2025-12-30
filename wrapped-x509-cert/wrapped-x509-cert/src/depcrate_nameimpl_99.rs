// Generated macro for impl_99 (impl)
macro_rules! Depcrate_nameimpl_99 {
() => {
// Module: crate::name
// Provides: {"impl_99"}
// Dependencies: {}
# [doc = " Parse a [`Name`] string."] # [doc = ""] # [doc = " Follows the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl FromStr for Name { type Err = der :: Error ; fn from_str (s : & str) -> der :: Result < Self > { Ok (Self (RdnSequence :: from_str (s) ?)) } }
};
}
