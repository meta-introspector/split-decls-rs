// Generated macro for impl_34 (impl)
macro_rules! Depcrate_attrimpl_34 {
() => {
// Module: crate::attr
// Provides: {"impl_34"}
// Dependencies: {}
# [doc = " Parse an [`AttributeTypeAndValue`] string."] # [doc = ""] # [doc = " This function follows the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl FromStr for AttributeTypeAndValue { type Err = Error ; fn from_str (s : & str) -> der :: Result < Self > { let idx = s . find ('=') . ok_or_else (| | Error :: from (ErrorKind :: Failed)) ? ; let (key , val) = s . split_at (idx) ; let val = & val [1 ..] ; let oid = match DB . by_name (key) { Some (oid) => * oid , None => ObjectIdentifier :: new (key) ? , } ; match val . strip_prefix ('#') { Some (val) => Self :: from_hex (oid , val) , None => Self :: from_delimited_str (oid , val) , } } }
};
}
