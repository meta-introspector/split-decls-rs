// Generated macro for configuration_snippet_tests (function)
macro_rules! Depcrate_test_configuration_snippetconfiguration_snippet_tests {
() => {
// Module: crate::test::configuration_snippet
// Provides: {"configuration_snippet_tests"}
// Dependencies: {}
# [test] fn configuration_snippet_tests () { super :: init_log () ; let blocks = get_code_blocks () ; let failures = blocks . iter () . filter (| block | ! block . fmt_skip ()) . map (ConfigCodeBlock :: formatted_is_idempotent) . fold (0 , | acc , r | acc + (! r as u32)) ; println ! ("Ran {} configurations tests." , blocks . len ()) ; assert_eq ! (failures , 0 , "{failures} configurations tests failed") ; }
};
}
