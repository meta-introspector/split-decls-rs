// Generated macro for IS_PUBKEY_HASH_PROP_ID (function)
macro_rules! Depcrate_um_wincryptIS_PUBKEY_HASH_PROP_ID {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_PUBKEY_HASH_PROP_ID"}
// Dependencies: {}
# [inline] pub fn IS_PUBKEY_HASH_PROP_ID (X : DWORD) -> bool { CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == X || CERT_PIN_SHA256_HASH_PROP_ID == X || CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == X }
};
}
