// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
# [doc = " Given an [`EphemeralSecret`] Key, compute the corresponding public key"] # [doc = " using the generator specified in RFC7748"] impl From < & EphemeralSecret > for PublicKey { fn from (secret : & EphemeralSecret) -> PublicKey { let secret = secret . as_scalar () ; let point = & MontgomeryPoint :: GENERATOR * & secret ; PublicKey (point) } }
};
}
