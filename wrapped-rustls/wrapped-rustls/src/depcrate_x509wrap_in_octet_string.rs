// Generated macro for wrap_in_octet_string (function)
macro_rules! Depcrate_x509wrap_in_octet_string {
() => {
// Module: crate::x509
// Provides: {"wrap_in_octet_string"}
// Dependencies: {}
# [doc = " Prepend stuff to `bytes` to put it in a DER OCTET STRING."] # [cfg_attr (not (feature = "ring") , expect (dead_code))] pub (crate) fn wrap_in_octet_string (bytes : & [u8]) -> Vec < u8 > { asn1_wrap (DER_OCTET_STRING_TAG , bytes , & []) }
};
}
