// Generated macro for impl_39 (impl)
macro_rules! Depcrate_cert_contextimpl_39 {
() => {
// Module: crate::cert_context
// Provides: {"impl_39"}
// Dependencies: {}
# [allow (missing_docs)] impl HashAlgorithm { pub fn md5 () -> HashAlgorithm { HashAlgorithm (Cryptography :: ALG_CLASS_HASH | Cryptography :: ALG_TYPE_ANY | Cryptography :: ALG_SID_MD5 , 16 ,) } pub fn sha1 () -> HashAlgorithm { HashAlgorithm (Cryptography :: ALG_CLASS_HASH | Cryptography :: ALG_TYPE_ANY | Cryptography :: ALG_SID_SHA1 , 20 ,) } pub fn sha256 () -> HashAlgorithm { HashAlgorithm (Cryptography :: ALG_CLASS_HASH | Cryptography :: ALG_TYPE_ANY | Cryptography :: ALG_SID_SHA_256 , 32 ,) } pub fn sha384 () -> HashAlgorithm { HashAlgorithm (Cryptography :: ALG_CLASS_HASH | Cryptography :: ALG_TYPE_ANY | Cryptography :: ALG_SID_SHA_384 , 48 ,) } pub fn sha512 () -> HashAlgorithm { HashAlgorithm (Cryptography :: ALG_CLASS_HASH | Cryptography :: ALG_TYPE_ANY | Cryptography :: ALG_SID_SHA_512 , 64 ,) } }
};
}
