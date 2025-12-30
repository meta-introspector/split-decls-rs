// Generated macro for IS_CERT_EXCLUDED_SUBTREE (function)
macro_rules! Depcrate_um_wincryptIS_CERT_EXCLUDED_SUBTREE {
() => {
// Module: crate::um::wincrypt
// Provides: {"IS_CERT_EXCLUDED_SUBTREE"}
// Dependencies: {}
# [inline] pub fn IS_CERT_EXCLUDED_SUBTREE (X : DWORD) -> bool { 0 != (X & CERT_EXCLUDED_SUBTREE_BIT) }
};
}
