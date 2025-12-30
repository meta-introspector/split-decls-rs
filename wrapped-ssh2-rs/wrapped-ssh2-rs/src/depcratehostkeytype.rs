// Generated macro for HostKeyType (enum)
macro_rules! DepcrateHostKeyType {
() => {
// Module: crate
// Provides: {"HostKeyType"}
// Dependencies: {}
# [allow (missing_docs)] # [derive (Copy , Clone , Debug)] pub enum HostKeyType { Unknown = raw :: LIBSSH2_HOSTKEY_TYPE_UNKNOWN as isize , Rsa = raw :: LIBSSH2_HOSTKEY_TYPE_RSA as isize , Dss = raw :: LIBSSH2_HOSTKEY_TYPE_DSS as isize , Ecdsa256 = raw :: LIBSSH2_HOSTKEY_TYPE_ECDSA_256 as isize , Ecdsa384 = raw :: LIBSSH2_HOSTKEY_TYPE_ECDSA_384 as isize , Ecdsa521 = raw :: LIBSSH2_HOSTKEY_TYPE_ECDSA_521 as isize , Ed25519 = raw :: LIBSSH2_HOSTKEY_TYPE_ED25519 as isize , }
};
}
