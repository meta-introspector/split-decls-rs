macro_rules! deps {
    () => {
        Version!();
        Builder!();
        Uuid!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Uuid { # [doc = " Creates a UUID using a name from a namespace, based on the MD5"] # [doc = " hash."] # [doc = ""] # [doc = " A number of namespaces are available as constants in this crate:"] # [doc = ""] # [doc = " * [`NAMESPACE_DNS`]"] # [doc = " * [`NAMESPACE_OID`]"] # [doc = " * [`NAMESPACE_URL`]"] # [doc = " * [`NAMESPACE_X500`]"] # [doc = ""] # [doc = " Note that usage of this method requires the `v3` feature of this crate"] # [doc = " to be enabled."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Generating a MD5 DNS UUID for `rust-lang.org`:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{Uuid, Version};"] # [doc = " let uuid = Uuid::new_v3(&Uuid::NAMESPACE_DNS, b\"rust-lang.org\");"] # [doc = ""] # [doc = " assert_eq!(Some(Version::Md5), uuid.get_version());"] # [doc = " ```"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [UUID Version 3 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.3)"] # [doc = " * [Name-Based UUID Generation in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.5)"] # [doc = ""] # [doc = " [`NAMESPACE_DNS`]: #associatedconstant.NAMESPACE_DNS"] # [doc = " [`NAMESPACE_OID`]: #associatedconstant.NAMESPACE_OID"] # [doc = " [`NAMESPACE_URL`]: #associatedconstant.NAMESPACE_URL"] # [doc = " [`NAMESPACE_X500`]: #associatedconstant.NAMESPACE_X500"] pub fn new_v3 (namespace : & Uuid , name : & [u8]) -> Uuid { crate :: Builder :: from_md5_bytes (crate :: md5 :: hash (namespace . as_bytes () , name)) . into_uuid () } }
    };
}

impl_100!()