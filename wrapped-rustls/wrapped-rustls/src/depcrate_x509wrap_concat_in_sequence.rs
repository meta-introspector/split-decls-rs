// Generated macro for wrap_concat_in_sequence (function)
macro_rules! Depcrate_x509wrap_concat_in_sequence {
() => {
// Module: crate::x509
// Provides: {"wrap_concat_in_sequence"}
// Dependencies: {}
# [doc = " Prepend stuff to `bytes_a` + `bytes_b` to put it in a DER SEQUENCE."] # [cfg_attr (not (feature = "ring") , expect (dead_code))] pub (crate) fn wrap_concat_in_sequence (bytes_a : & [u8] , bytes_b : & [u8]) -> Vec < u8 > { asn1_wrap (DER_SEQUENCE_TAG , bytes_a , bytes_b) }
};
}
