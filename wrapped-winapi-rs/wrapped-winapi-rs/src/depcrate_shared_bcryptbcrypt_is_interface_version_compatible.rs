// Generated macro for BCRYPT_IS_INTERFACE_VERSION_COMPATIBLE (function)
macro_rules! Depcrate_shared_bcryptBCRYPT_IS_INTERFACE_VERSION_COMPATIBLE {
() => {
// Module: crate::shared::bcrypt
// Provides: {"BCRYPT_IS_INTERFACE_VERSION_COMPATIBLE"}
// Dependencies: {}
# [inline] pub fn BCRYPT_IS_INTERFACE_VERSION_COMPATIBLE (loader : BCRYPT_INTERFACE_VERSION , provider : BCRYPT_INTERFACE_VERSION ,) -> bool { loader . MajorVersion <= provider . MajorVersion }
};
}
