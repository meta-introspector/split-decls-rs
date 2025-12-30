// Generated macro for ring (module)
macro_rules! Depcratering {
() => {
// Module: crate
// Provides: {"ring"}
// Dependencies: {}
# [cfg (feature = "ring")] # [doc = " Signature verification algorithm implementations using the *ring* crypto library."] pub mod ring { pub use super :: ring_algs :: { ECDSA_P256_SHA256 , ECDSA_P256_SHA384 , ECDSA_P384_SHA256 , ECDSA_P384_SHA384 , ED25519 , } ; # [cfg (feature = "alloc")] pub use super :: ring_algs :: { RSA_PKCS1_2048_8192_SHA256 , RSA_PKCS1_2048_8192_SHA256_ABSENT_PARAMS , RSA_PKCS1_2048_8192_SHA384 , RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS , RSA_PKCS1_2048_8192_SHA512 , RSA_PKCS1_2048_8192_SHA512_ABSENT_PARAMS , RSA_PKCS1_3072_8192_SHA384 , RSA_PSS_2048_8192_SHA256_LEGACY_KEY , RSA_PSS_2048_8192_SHA384_LEGACY_KEY , RSA_PSS_2048_8192_SHA512_LEGACY_KEY , } ; }
};
}
