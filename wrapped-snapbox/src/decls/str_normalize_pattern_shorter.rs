macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_pattern_shorter {
    () => {
        deps!();
        # [test] fn str_normalize_pattern_shorter () { let input = "Hello\nWorld" ; let pattern = "Hello\n" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_pattern_shorter!()