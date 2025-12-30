// Generated macro for is_nfc (function)
macro_rules! Depcrate_quick_checkis_nfc {
() => {
// Module: crate::quick_check
// Provides: {"is_nfc"}
// Dependencies: {}
# [doc = " Authoritatively check if a string is in NFC."] # [inline] pub fn is_nfc (s : & str) -> bool { match is_nfc_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfc ()) , } }
};
}
