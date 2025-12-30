// Generated macro for aws_lc_rs (module)
macro_rules! Depcrateaws_lc_rs {
() => {
// Module: crate
// Provides: {"aws_lc_rs"}
// Dependencies: {}
# [cfg (feature = "aws-lc-rs")] # [doc = " Signature verification algorithm implementations using the aws-lc-rs crypto library."] pub mod aws_lc_rs { pub use super :: aws_lc_rs_algs :: { ECDSA_P256_SHA256 , ECDSA_P256_SHA384 , ECDSA_P256_SHA512 , ECDSA_P384_SHA256 , ECDSA_P384_SHA384 , ECDSA_P384_SHA512 , ECDSA_P521_SHA256 , ECDSA_P521_SHA384 , ECDSA_P521_SHA512 , ED25519 , RSA_PKCS1_2048_8192_SHA256 , RSA_PKCS1_2048_8192_SHA256_ABSENT_PARAMS , RSA_PKCS1_2048_8192_SHA384 , RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS , RSA_PKCS1_2048_8192_SHA512 , RSA_PKCS1_2048_8192_SHA512_ABSENT_PARAMS , RSA_PKCS1_3072_8192_SHA384 , RSA_PSS_2048_8192_SHA256_LEGACY_KEY , RSA_PSS_2048_8192_SHA384_LEGACY_KEY , RSA_PSS_2048_8192_SHA512_LEGACY_KEY , } ; # [cfg (all (feature = "aws-lc-rs-unstable" , not (feature = "aws-lc-rs-fips")))] pub use super :: aws_lc_rs_algs :: { ML_DSA_44 , ML_DSA_65 , ML_DSA_87 } ; }
};
}
