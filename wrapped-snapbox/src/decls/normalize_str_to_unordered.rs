macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! normalize_str_to_unordered {
    () => {
        deps!();
        fn normalize_str_to_unordered (actual : & str , expected : & str) -> String { if actual == expected { return actual . to_owned () ; } let mut normalized : Vec < & str > = Vec :: new () ; let mut actual_lines : Vec < _ > = crate :: utils :: LinesWithTerminator :: new (actual) . collect () ; let mut expected_lines : Vec < _ > = crate :: utils :: LinesWithTerminator :: new (expected) . collect () ; expected_lines . retain (| expected_line | { let mut matched = false ; actual_lines . retain (| actual_line | { if ! matched && actual_line == expected_line { matched = true ; false } else { true } }) ; if matched { normalized . push (expected_line) ; } ! matched }) ; for actual_line in & actual_lines { normalized . push (actual_line) ; } normalized . join ("") }
    };
}

normalize_str_to_unordered!()