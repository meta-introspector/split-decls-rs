// Generated macro for DEFAULT_KX_GROUPS (static)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_KX_GROUPS {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_KX_GROUPS"}
// Dependencies: {}
# [doc = " A list of the default key exchange groups supported by this provider."] # [doc = ""] # [doc = " This does not contain MLKEM768; by default MLKEM768 is only offered"] # [doc = " in hybrid with X25519."] pub static DEFAULT_KX_GROUPS : & [& dyn SupportedKxGroup] = & [kx_group :: X25519MLKEM768 , # [cfg (not (feature = "fips"))] kx_group :: X25519 , kx_group :: SECP256R1 , kx_group :: SECP384R1 ,] ;
};
}
