// Generated macro for IS_CHAIN_HASH_PROP_ID (function)
macro_rules! Depcrate_um_wincryptIS_CHAIN_HASH_PROP_ID {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_CHAIN_HASH_PROP_ID"}
// Dependencies: {}
# [inline] pub fn IS_CHAIN_HASH_PROP_ID (X : DWORD) -> bool { CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == X || CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == X || CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID == X || CERT_SUBJECT_NAME_MD5_HASH_PROP_ID == X }
};
}
