// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (feature = "static_secrets")] impl From < & StaticSecret > for PublicKey { # [doc = " Given an x448 [`StaticSecret`] key, compute its corresponding [`PublicKey`]."] fn from (secret : & StaticSecret) -> PublicKey { let secret = secret . as_scalar () ; let point = & MontgomeryPoint :: GENERATOR * & secret ; PublicKey (point) } }
};
}
