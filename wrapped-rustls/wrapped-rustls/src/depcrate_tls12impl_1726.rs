// Generated macro for impl_1726 (impl)
macro_rules! Depcrate_tls12impl_1726 {
() => {
// Module: crate::tls12
// Provides: {"impl_1726"}
// Dependencies: {}
impl AsRef < [u8] > for Seed { # [doc = " This is guaranteed to return a non-empty slice."] fn as_ref (& self) -> & [u8] { match self { Self :: Ems (seed) => seed . as_ref () , Self :: Randoms (randoms) => randoms . as_ref () , } } }
};
}
