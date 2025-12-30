// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (feature = "static_secrets")] impl From < [u8 ; 56] > for StaticSecret { # [doc = " Load a secret key from a byte array."] fn from (bytes : [u8 ; 56]) -> StaticSecret { StaticSecret :: new (bytes . into ()) } }
};
}
