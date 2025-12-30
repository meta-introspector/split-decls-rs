// Generated macro for read_tag_and_get_value (function)
macro_rules! Depcrate_derread_tag_and_get_value {
() => {
// Module: crate::der
// Provides: {"read_tag_and_get_value"}
// Dependencies: {}
# [inline (always)] pub (crate) fn read_tag_and_get_value < 'a > (input : & mut untrusted :: Reader < 'a > ,) -> Result < (u8 , untrusted :: Input < 'a >) , Error > { read_tag_and_get_value_limited (input , TWO_BYTE_DER_SIZE) }
};
}
