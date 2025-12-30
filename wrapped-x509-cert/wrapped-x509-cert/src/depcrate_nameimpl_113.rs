// Generated macro for impl_113 (impl)
macro_rules! Depcrate_nameimpl_113 {
() => {
// Module: crate::name
// Provides: {"impl_113"}
// Dependencies: {}
# [doc = " Serializes the structure according to the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl fmt :: Display for RelativeDistinguishedName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , atv) in self . 0 . iter () . enumerate () { match i { 0 => write ! (f , "{atv}") ? , _ => write ! (f , "+{atv}") ? , } } Ok (()) } }
};
}
