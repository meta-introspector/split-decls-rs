// Generated macro for IS_CERT_HASH_PROP_ID (function)
macro_rules! Depcrate_um_wincryptIS_CERT_HASH_PROP_ID {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_CERT_HASH_PROP_ID"}
// Dependencies: {}
# [inline] pub fn IS_CERT_HASH_PROP_ID (X : DWORD) -> bool { CERT_SHA1_HASH_PROP_ID == X || CERT_MD5_HASH_PROP_ID == X || CERT_SHA256_HASH_PROP_ID == X || CERT_SIGNATURE_HASH_PROP_ID == X }
};
}
