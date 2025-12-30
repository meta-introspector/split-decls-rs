// Generated macro for impl_43 (impl)
macro_rules! Depcrate_hostimpl_43 {
() => {
// Module: crate::host
// Provides: {"impl_43"}
// Dependencies: {}
impl Host < & str > { # [doc = " Return a copy of `self` that owns an allocated `String` but does not borrow an `&Url`."] pub fn to_owned (& self) -> Host < String > { match * self { Host :: Domain (domain) => Host :: Domain (domain . to_owned ()) , Host :: Ipv4 (address) => Host :: Ipv4 (address) , Host :: Ipv6 (address) => Host :: Ipv6 (address) , } } }
};
}
