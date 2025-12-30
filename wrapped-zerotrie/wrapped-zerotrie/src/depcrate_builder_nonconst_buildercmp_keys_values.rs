// Generated macro for cmp_keys_values (function)
macro_rules! Depcrate_builder_nonconst_buildercmp_keys_values {
() => {
// Module: crate::builder::nonconst::builder
// Provides: {"cmp_keys_values"}
// Dependencies: {}
fn cmp_keys_values (options : ZeroTrieBuilderOptions , a : (& [u8] , usize) , b : (& [u8] , usize) ,) -> Ordering { if matches ! (options . case_sensitivity , CaseSensitivity :: Sensitive) { a . 0 . cmp (b . 0) } else { let a_iter = a . 0 . iter () . map (| x | x . to_ascii_lowercase ()) ; let b_iter = b . 0 . iter () . map (| x | x . to_ascii_lowercase ()) ; Iterator :: cmp (a_iter , b_iter) } . then_with (| | a . 1 . cmp (& b . 1)) }
};
}
