// Generated macro for IS_CERT_RDN_CHAR_STRING (function)
macro_rules! Depcrate_um_wincryptIS_CERT_RDN_CHAR_STRING {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_CERT_RDN_CHAR_STRING"}
// Dependencies: {}
# [inline] pub fn IS_CERT_RDN_CHAR_STRING (X : DWORD) -> bool { (X & CERT_RDN_TYPE_MASK) >= CERT_RDN_NUMERIC_STRING }
};
}
