macro_rules! deps {
    () => {
        Redactions!();
        LinesWithTerminator!();
    };
}

macro_rules! normalize_str_to_unordered_redactions {
    () => {
        deps!();
        fn normalize_str_to_unordered_redactions (actual : & str , expected : & str , substitutions : & Redactions ,) -> String { if actual == expected { return actual . to_owned () ; } let mut normalized : Vec < & str > = Vec :: new () ; let mut actual_lines : Vec < _ > = crate :: utils :: LinesWithTerminator :: new (actual) . collect () ; let mut expected_lines : Vec < _ > = crate :: utils :: LinesWithTerminator :: new (expected) . collect () ; let mut elided = false ; expected_lines . retain (| expected_line | { let mut matched = false ; if is_line_elide (expected_line) { matched = true ; elided = true ; } else { actual_lines . retain (| actual_line | { if ! matched && line_matches (actual_line , expected_line , substitutions) { matched = true ; false } else { true } }) ; } if matched { normalized . push (expected_line) ; } ! matched }) ; if ! elided { for actual_line in & actual_lines { normalized . push (actual_line) ; } } normalized . join ("") }
    };
}

normalize_str_to_unordered_redactions!();