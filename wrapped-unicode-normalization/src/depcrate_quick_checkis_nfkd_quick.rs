// Generated macro for is_nfkd_quick (function)
macro_rules! Depcrate_quick_checkis_nfkd_quick {
() => {
// Module: crate::quick_check
// Provides: {"is_nfkd_quick"}
// Dependencies: {}
# [doc = " Quickly check if a string is in NFKD."] # [inline] pub fn is_nfkd_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfkd , false) }
};
}
