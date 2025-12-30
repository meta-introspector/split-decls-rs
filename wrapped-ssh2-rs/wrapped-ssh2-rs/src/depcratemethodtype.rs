// Generated macro for MethodType (enum)
macro_rules! DepcrateMethodType {
() => {
// Module: crate
// Provides: {"MethodType"}
// Dependencies: {}
# [allow (missing_docs)] # [derive (Copy , Clone)] pub enum MethodType { Kex = raw :: LIBSSH2_METHOD_KEX as isize , HostKey = raw :: LIBSSH2_METHOD_HOSTKEY as isize , CryptCs = raw :: LIBSSH2_METHOD_CRYPT_CS as isize , CryptSc = raw :: LIBSSH2_METHOD_CRYPT_SC as isize , MacCs = raw :: LIBSSH2_METHOD_MAC_CS as isize , MacSc = raw :: LIBSSH2_METHOD_MAC_SC as isize , CompCs = raw :: LIBSSH2_METHOD_COMP_CS as isize , CompSc = raw :: LIBSSH2_METHOD_COMP_SC as isize , LangCs = raw :: LIBSSH2_METHOD_LANG_CS as isize , LangSc = raw :: LIBSSH2_METHOD_LANG_SC as isize , SignAlgo = raw :: LIBSSH2_METHOD_SIGN_ALGO as isize , }
};
}
