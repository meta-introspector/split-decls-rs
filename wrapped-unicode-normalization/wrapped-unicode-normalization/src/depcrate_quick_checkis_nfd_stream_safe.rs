// Generated macro for is_nfd_stream_safe (function)
macro_rules! Depcrate_quick_checkis_nfd_stream_safe {
() => {
// Module: crate::quick_check
// Provides: {"is_nfd_stream_safe"}
// Dependencies: {}
# [doc = " Authoritatively check if a string is Stream-Safe NFD."] # [inline] pub fn is_nfd_stream_safe (s : & str) -> bool { match is_nfd_stream_safe_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . stream_safe () . nfd ()) , } }
};
}
