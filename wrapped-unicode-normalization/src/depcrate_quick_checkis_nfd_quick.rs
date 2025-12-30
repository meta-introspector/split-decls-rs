// Generated macro for is_nfd_quick (function)
macro_rules! Depcrate_quick_checkis_nfd_quick {
() => {
// Module: crate::quick_check
// Provides: {"is_nfd_quick"}
// Dependencies: {}
# [doc = " Quickly check if a string is in NFD."] # [inline] pub fn is_nfd_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfd , false) }
};
}
