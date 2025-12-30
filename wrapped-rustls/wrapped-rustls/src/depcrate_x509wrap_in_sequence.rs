// Generated macro for wrap_in_sequence (function)
macro_rules! Depcrate_x509wrap_in_sequence {
() => {
// Module: crate::x509
// Provides: {"wrap_in_sequence"}
// Dependencies: {}
# [doc = " Prepend stuff to `bytes` to put it in a DER SEQUENCE."] pub (crate) fn wrap_in_sequence (bytes : & [u8]) -> Vec < u8 > { asn1_wrap (DER_SEQUENCE_TAG , bytes , & []) }
};
}
