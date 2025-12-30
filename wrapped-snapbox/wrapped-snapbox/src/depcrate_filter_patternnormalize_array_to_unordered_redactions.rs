// Generated macro for normalize_array_to_unordered_redactions (function)
macro_rules! Depcrate_filter_patternnormalize_array_to_unordered_redactions {
() => {
// Module: crate::filter::pattern
// Provides: {"normalize_array_to_unordered_redactions"}
// Dependencies: {}
# [cfg (feature = "structured-data")] fn normalize_array_to_unordered_redactions (actual : & [serde_json :: Value] , expected : & [serde_json :: Value] , substitutions : & Redactions ,) -> Vec < serde_json :: Value > { if actual == expected { return actual . to_owned () ; } let mut normalized : Vec < serde_json :: Value > = Vec :: new () ; let mut actual_values = actual . to_owned () ; let mut expected_values = expected . to_owned () ; let mut elided = false ; expected_values . retain (| expected_value | { let mut matched = false ; if expected_value == VALUE_WILDCARD { matched = true ; elided = true ; } else { actual_values . retain (| actual_value | { let mut normalized_actual_value = actual_value . clone () ; normalize_value_to_unordered_redactions (& mut normalized_actual_value , expected_value , substitutions ,) ; if ! matched && normalized_actual_value == * expected_value { matched = true ; false } else { true } }) ; } if matched { normalized . push (expected_value . clone ()) ; } ! matched }) ; if ! elided { for actual_value in actual_values { normalized . push (actual_value) ; } } normalized }
};
}
