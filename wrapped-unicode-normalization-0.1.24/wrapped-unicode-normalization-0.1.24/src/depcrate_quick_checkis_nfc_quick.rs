// Generated macro for is_nfc_quick (function)
macro_rules! Depcrate_quick_checkis_nfc_quick {
() => {
// Module: crate::quick_check
// Provides: {"is_nfc_quick"}
// Dependencies: {}
# [doc = " Quickly check if a string is in NFC, potentially returning"] # [doc = " `IsNormalized::Maybe` if further checks are necessary.  In this case a check"] # [doc = " like `s.chars().nfc().eq(s.chars())` should suffice."] # [inline] pub fn is_nfc_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfc , false) }
};
}
