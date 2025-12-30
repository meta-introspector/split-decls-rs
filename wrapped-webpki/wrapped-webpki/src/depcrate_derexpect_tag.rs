// Generated macro for expect_tag (function)
macro_rules! Depcrate_derexpect_tag {
() => {
// Module: crate::der
// Provides: {"expect_tag"}
// Dependencies: {}
pub (crate) fn expect_tag < 'a > (input : & mut untrusted :: Reader < 'a > , tag : Tag ,) -> Result < untrusted :: Input < 'a > , Error > { let (actual_tag , value) = read_tag_and_get_value_limited (input , TWO_BYTE_DER_SIZE) ? ; if usize :: from (tag) != usize :: from (actual_tag) { return Err (Error :: BadDer) ; } Ok (value) }
};
}
