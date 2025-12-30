// Generated macro for test_convert_constant_to_num_denom_strings (function)
macro_rules! Depcrate_units_helperstest_convert_constant_to_num_denom_strings {
() => {
// Module: crate::units::helpers
// Provides: {"test_convert_constant_to_num_denom_strings"}
// Dependencies: {}
# [test] fn test_convert_constant_to_num_denom_strings () { let input = "1/2" ; let expected = (vec ! ["1" . to_string ()] , vec ! ["2" . to_string ()]) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "1 * 2 / 3 * ft_to_m" ; let expected = (vec ! ["1" . to_string () , "2" . to_string ()] , vec ! ["3" . to_string () , "ft_to_m" . to_string ()] ,) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "/2" ; let expected = (vec ! ["1" . to_string ()] , vec ! ["2" . to_string ()]) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "2" ; let expected = (vec ! ["2" . to_string ()] , vec ! ["1" . to_string ()]) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "2/" ; let expected = (vec ! ["2" . to_string ()] , vec ! ["1" . to_string ()]) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "1E2" ; let expected = (vec ! ["1E2" . to_string ()] , vec ! ["1" . to_string ()]) ; let actual = split_unit_term (input) . unwrap () ; assert_eq ! (expected , actual) ; let input = "1 2 * 3" ; let actual = split_unit_term (input) ; assert ! (actual . is_err ()) ; }
};
}
