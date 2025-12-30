// Generated macro for is_nfc_stream_safe_quick (function)
macro_rules! Depcrate_quick_checkis_nfc_stream_safe_quick {
() => {
// Module: crate::quick_check
// Provides: {"is_nfc_stream_safe_quick"}
// Dependencies: {}
# [doc = " Quickly check if a string is Stream-Safe NFC."] # [inline] pub fn is_nfc_stream_safe_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfc , true) }
};
}
