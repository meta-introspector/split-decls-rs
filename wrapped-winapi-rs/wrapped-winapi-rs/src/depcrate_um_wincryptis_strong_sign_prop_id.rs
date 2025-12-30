// Generated macro for IS_STRONG_SIGN_PROP_ID (function)
macro_rules! Depcrate_um_wincryptIS_STRONG_SIGN_PROP_ID {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_STRONG_SIGN_PROP_ID"}
// Dependencies: {}
# [inline] pub fn IS_STRONG_SIGN_PROP_ID (X : DWORD) -> bool { CERT_SIGN_HASH_CNG_ALG_PROP_ID == X || CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID == X || CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID == X }
};
}
