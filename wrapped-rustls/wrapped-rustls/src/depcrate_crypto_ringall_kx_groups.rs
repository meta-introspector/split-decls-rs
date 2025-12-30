// Generated macro for ALL_KX_GROUPS (static)
macro_rules! Depcrate_crypto_ringALL_KX_GROUPS {
() => {
// Module: crate::crypto::ring
// Provides: {"ALL_KX_GROUPS"}
// Dependencies: {}
# [doc = " A list of all the key exchange groups supported by this provider."] pub static ALL_KX_GROUPS : & [& dyn SupportedKxGroup] = & [kx_group :: X25519 , kx_group :: SECP256R1 , kx_group :: SECP384R1] ;
};
}
