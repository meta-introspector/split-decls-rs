// Generated macro for normalize_value_to_unordered (function)
macro_rules! Depcrate_filter_patternnormalize_value_to_unordered {
() => {
// Module: crate::filter::pattern
// Provides: {"normalize_value_to_unordered"}
// Dependencies: {}
# [cfg (feature = "structured-data")] fn normalize_value_to_unordered (actual : & mut serde_json :: Value , expected : & serde_json :: Value) { use serde_json :: Value :: { Array , Object , String } ; match (actual , expected) { (String (act) , String (exp)) => { * act = normalize_str_to_unordered (act , exp) ; } (Array (act) , Array (exp)) => { let mut actual_values = std :: mem :: take (act) ; let mut expected_values = exp . clone () ; expected_values . retain (| expected_value | { let mut matched = false ; actual_values . retain (| actual_value | { if ! matched && actual_value == expected_value { matched = true ; false } else { true } }) ; if matched { act . push (expected_value . clone ()) ; } ! matched }) ; for actual_value in actual_values { act . push (actual_value) ; } } (Object (act) , Object (exp)) => { for (actual_key , mut actual_value) in std :: mem :: replace (act , serde_json :: Map :: new ()) { if let Some (expected_value) = exp . get (& actual_key) { normalize_value_to_unordered (& mut actual_value , expected_value) ; } act . insert (actual_key , actual_value) ; } } (_ , _) => { } } }
};
}
