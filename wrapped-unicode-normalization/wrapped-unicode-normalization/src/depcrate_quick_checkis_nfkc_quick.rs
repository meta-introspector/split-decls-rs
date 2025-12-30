// Generated macro for is_nfkc_quick (function)
macro_rules! Depcrate_quick_checkis_nfkc_quick {
() => {
// Module: crate::quick_check
// Provides: {"is_nfkc_quick"}
// Dependencies: {}
# [doc = " Quickly check if a string is in NFKC."] # [inline] pub fn is_nfkc_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfkc , false) }
};
}
