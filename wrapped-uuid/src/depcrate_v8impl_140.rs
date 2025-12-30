// Generated macro for impl_140 (impl)
macro_rules! Depcrate_v8impl_140 {
() => {
// Module: crate::v8
// Provides: {"impl_140"}
// Dependencies: {}
impl Uuid { # [doc = " Creates a custom UUID comprised almost entirely of user-supplied bytes."] # [doc = ""] # [doc = " This will inject the UUID Version at 4 bits starting at the 48th bit"] # [doc = " and the Variant into 2 bits 64th bit. Any existing bits in the user-supplied bytes"] # [doc = " at those locations will be overridden."] # [doc = ""] # [doc = " Note that usage of this method requires the `v8` feature of this crate"] # [doc = " to be enabled."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{Uuid, Version};"] # [doc = " let buf: [u8; 16] = *b\"abcdefghijklmnop\";"] # [doc = " let uuid = Uuid::new_v8(buf);"] # [doc = ""] # [doc = " assert_eq!(Some(Version::Custom), uuid.get_version());"] # [doc = " ```"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [UUID Version 8 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.8)"] pub const fn new_v8 (buf : [u8 ; 16]) -> Uuid { Builder :: from_custom_bytes (buf) . into_uuid () } }
};
}
