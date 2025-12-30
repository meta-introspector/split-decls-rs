// Generated macro for expect_tag_and_get_value_limited (function)
macro_rules! Depcrate_derexpect_tag_and_get_value_limited {
() => {
// Module: crate::der
// Provides: {"expect_tag_and_get_value_limited"}
// Dependencies: {}
# [inline (always)] pub (crate) fn expect_tag_and_get_value_limited < 'a > (input : & mut untrusted :: Reader < 'a > , tag : Tag , size_limit : usize ,) -> Result < untrusted :: Input < 'a > , Error > { let (actual_tag , inner) = read_tag_and_get_value_limited (input , size_limit) ? ; if usize :: from (tag) != usize :: from (actual_tag) { return Err (Error :: BadDer) ; } Ok (inner) }
};
}
