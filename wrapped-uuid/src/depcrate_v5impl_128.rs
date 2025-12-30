// Generated macro for impl_128 (impl)
macro_rules! Depcrate_v5impl_128 {
() => {
// Module: crate::v5
// Provides: {"impl_128"}
// Dependencies: {}
impl Uuid { # [doc = " Creates a UUID using a name from a namespace, based on the SHA-1 hash."] # [doc = ""] # [doc = " A number of namespaces are available as constants in this crate:"] # [doc = ""] # [doc = " * [`NAMESPACE_DNS`]"] # [doc = " * [`NAMESPACE_OID`]"] # [doc = " * [`NAMESPACE_URL`]"] # [doc = " * [`NAMESPACE_X500`]"] # [doc = ""] # [doc = " Note that usage of this method requires the `v5` feature of this crate"] # [doc = " to be enabled."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Generating a SHA1 DNS UUID for `rust-lang.org`:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{Uuid, Version};"] # [doc = " let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b\"rust-lang.org\");"] # [doc = ""] # [doc = " assert_eq!(Some(Version::Sha1), uuid.get_version());"] # [doc = " ```"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [UUID Version 5 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.5)"] # [doc = " * [Name-Based UUID Generation in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.5)"] # [doc = ""] # [doc = " [`NAMESPACE_DNS`]: struct.Uuid.html#associatedconstant.NAMESPACE_DNS"] # [doc = " [`NAMESPACE_OID`]: struct.Uuid.html#associatedconstant.NAMESPACE_OID"] # [doc = " [`NAMESPACE_URL`]: struct.Uuid.html#associatedconstant.NAMESPACE_URL"] # [doc = " [`NAMESPACE_X500`]: struct.Uuid.html#associatedconstant.NAMESPACE_X500"] pub fn new_v5 (namespace : & Uuid , name : & [u8]) -> Uuid { crate :: Builder :: from_sha1_bytes (crate :: sha1 :: hash (namespace . as_bytes () , name)) . into_uuid () } }
};
}
