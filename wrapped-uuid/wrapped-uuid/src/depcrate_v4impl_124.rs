// Generated macro for impl_124 (impl)
macro_rules! Depcrate_v4impl_124 {
() => {
// Module: crate::v4
// Provides: {"impl_124"}
// Dependencies: {}
impl Uuid { # [doc = " Creates a random UUID."] # [doc = ""] # [doc = " This uses the [`getrandom`] crate to utilise the operating system's RNG"] # [doc = " as the source of random numbers. If you'd like to use a custom"] # [doc = " generator, don't use this method: generate random bytes using your"] # [doc = " custom generator and pass them to the"] # [doc = " [`uuid::Builder::from_random_bytes`][from_random_bytes] function"] # [doc = " instead."] # [doc = ""] # [doc = " Note that usage of this method requires the `v4` feature of this crate"] # [doc = " to be enabled."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{Uuid, Version};"] # [doc = " let uuid = Uuid::new_v4();"] # [doc = ""] # [doc = " assert_eq!(Some(Version::Random), uuid.get_version());"] # [doc = " ```"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [UUID Version 4 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.4)"] # [doc = ""] # [doc = " [`getrandom`]: https://crates.io/crates/getrandom"] # [doc = " [from_random_bytes]: struct.Builder.html#method.from_random_bytes"] pub fn new_v4 () -> Uuid { Uuid :: from_u128 (crate :: rng :: u128 () & 0xFFFFFFFFFFFF4FFFBFFFFFFFFFFFFFFF | 0x40008000000000000000 ,) } }
};
}
