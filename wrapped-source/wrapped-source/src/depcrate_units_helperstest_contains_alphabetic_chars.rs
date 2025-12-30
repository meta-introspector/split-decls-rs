// Generated macro for test_contains_alphabetic_chars (function)
macro_rules! Depcrate_units_helperstest_contains_alphabetic_chars {
() => {
// Module: crate::units::helpers
// Provides: {"test_contains_alphabetic_chars"}
// Dependencies: {}
# [test] fn test_contains_alphabetic_chars () { let input = "1" ; let expected = false ; let actual = contains_alphabetic_chars (input) ; assert_eq ! (expected , actual) ; let input = "ft_to_m" ; let expected = true ; let actual = contains_alphabetic_chars (input) ; assert_eq ! (expected , actual) ; let input = "1E2" ; let expected = true ; let actual = contains_alphabetic_chars (input) ; assert_eq ! (expected , actual) ; let input = "1.5E-2" ; let expected = true ; let actual = contains_alphabetic_chars (input) ; assert_eq ! (expected , actual) ; }
};
}
