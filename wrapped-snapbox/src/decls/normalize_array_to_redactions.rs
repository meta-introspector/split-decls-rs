macro_rules! deps {
    () => {
        Redactions!();
    };
}

macro_rules! normalize_array_to_redactions {
    () => {
        deps!();
        # [cfg (feature = "structured-data")] fn normalize_array_to_redactions (actual : & [serde_json :: Value] , expected : & [serde_json :: Value] , redactions : & Redactions ,) -> Vec < serde_json :: Value > { if actual == expected { return actual . to_vec () ; } let mut normalized : Vec < serde_json :: Value > = Vec :: new () ; let mut actual_index = 0 ; let mut expected = expected . iter () . peekable () ; while let Some (expected_elem) = expected . next () { if expected_elem == VALUE_WILDCARD { let Some (next_expected_elem) = expected . peek () else { normalized . push (expected_elem . clone ()) ; actual_index = actual . len () ; break ; } ; let Some (index_offset) = actual [actual_index ..] . iter () . position (| next_actual_elem | { let mut next_actual_elem = next_actual_elem . clone () ; normalize_value_to_redactions (& mut next_actual_elem , next_expected_elem , redactions ,) ; next_actual_elem == * * next_expected_elem }) else { break ; } ; normalized . push (expected_elem . clone ()) ; actual_index += index_offset ; } else { let Some (actual_elem) = actual . get (actual_index) else { break ; } ; actual_index += 1 ; let mut normalized_elem = actual_elem . clone () ; normalize_value_to_redactions (& mut normalized_elem , expected_elem , redactions) ; normalized . push (normalized_elem) ; } } normalized . extend (actual [actual_index ..] . iter () . cloned ()) ; normalized }
    };
}

normalize_array_to_redactions!()