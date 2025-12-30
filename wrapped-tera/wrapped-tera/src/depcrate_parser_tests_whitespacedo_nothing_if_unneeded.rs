// Generated macro for do_nothing_if_unneeded (function)
macro_rules! Depcrate_parser_tests_whitespacedo_nothing_if_unneeded {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"do_nothing_if_unneeded"}
// Dependencies: {}
# [test] fn do_nothing_if_unneeded () { let ast = vec ! [Node :: Text ("hey " . to_string ())] ; assert_eq ! (remove_whitespace (ast . clone () , None) , ast) ; }
};
}
