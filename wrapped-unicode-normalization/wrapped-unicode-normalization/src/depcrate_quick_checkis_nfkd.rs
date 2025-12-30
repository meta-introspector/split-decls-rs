// Generated macro for is_nfkd (function)
macro_rules! Depcrate_quick_checkis_nfkd {
() => {
// Module: crate::quick_check
// Provides: {"is_nfkd"}
// Dependencies: {}
# [doc = " Authoritatively check if a string is in NFKD."] # [inline] pub fn is_nfkd (s : & str) -> bool { match is_nfkd_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfkd ()) , } }
};
}
