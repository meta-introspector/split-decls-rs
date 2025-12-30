// Generated macro for kx_group (module)
macro_rules! Depcrate_crypto_ringkx_group {
() => {
// Module: crate::crypto::ring
// Provides: {"kx_group"}
// Dependencies: {}
# [doc = " All defined key exchange groups supported by *ring* appear in this module."] # [doc = ""] # [doc = " [`ALL_KX_GROUPS`] is provided as an array of all of these values."] # [doc = " [`DEFAULT_KX_GROUPS`] is provided as an array of this provider's defaults."] pub mod kx_group { pub use super :: kx :: { SECP256R1 , SECP384R1 , X25519 } ; }
};
}
