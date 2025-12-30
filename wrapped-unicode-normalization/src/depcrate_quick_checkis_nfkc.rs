// Generated macro for is_nfkc (function)
macro_rules! Depcrate_quick_checkis_nfkc {
() => {
// Module: crate::quick_check
// Provides: {"is_nfkc"}
// Dependencies: {}
# [doc = " Authoritatively check if a string is in NFKC."] # [inline] pub fn is_nfkc (s : & str) -> bool { match is_nfkc_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfkc ()) , } }
};
}
