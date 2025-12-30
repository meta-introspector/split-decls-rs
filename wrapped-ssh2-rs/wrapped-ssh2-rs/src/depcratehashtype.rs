// Generated macro for HashType (enum)
macro_rules! DepcrateHashType {
() => {
// Module: crate
// Provides: {"HashType"}
// Dependencies: {}
# [allow (missing_docs)] # [derive (Copy , Clone , Debug)] pub enum HashType { Md5 = raw :: LIBSSH2_HOSTKEY_HASH_MD5 as isize , Sha1 = raw :: LIBSSH2_HOSTKEY_HASH_SHA1 as isize , Sha256 = raw :: LIBSSH2_HOSTKEY_HASH_SHA256 as isize , }
};
}
