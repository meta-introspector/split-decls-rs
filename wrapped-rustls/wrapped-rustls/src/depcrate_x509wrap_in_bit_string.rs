// Generated macro for wrap_in_bit_string (function)
macro_rules! Depcrate_x509wrap_in_bit_string {
() => {
// Module: crate::x509
// Provides: {"wrap_in_bit_string"}
// Dependencies: {}
# [doc = " Prepend stuff to `bytes` to put it in a DER BIT STRING."] pub (crate) fn wrap_in_bit_string (bytes : & [u8]) -> Vec < u8 > { asn1_wrap (DER_BIT_STRING_TAG , & [0u8] , bytes) }
};
}
