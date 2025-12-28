macro_rules! parse_string_array {
    () => {
        fn parse_string_array (pair : Pair < Rule >) -> Vec < String > { let mut vals = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: string => { vals . push (replace_string_markers (p . as_span () . as_str ())) ; } _ => unreachable ! ("Got {:?} in parse_string_array" , p . as_rule ()) , } } vals }
    };
}

parse_string_array!();