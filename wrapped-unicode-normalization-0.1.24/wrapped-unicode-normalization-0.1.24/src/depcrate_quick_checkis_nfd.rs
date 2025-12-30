// Generated macro for is_nfd (function)
macro_rules! Depcrate_quick_checkis_nfd {
() => {
// Module: crate::quick_check
// Provides: {"is_nfd"}
// Dependencies: {}
# [doc = " Authoritatively check if a string is in NFD."] # [inline] pub fn is_nfd (s : & str) -> bool { match is_nfd_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfd ()) , } }
};
}
