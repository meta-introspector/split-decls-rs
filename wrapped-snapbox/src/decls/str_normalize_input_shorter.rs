macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_input_shorter {
    () => {
        deps!();
        # [test] fn str_normalize_input_shorter () { let input = "Hello\n" ; let pattern = "Hello\nWorld" ; let expected = "Hello\n" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_input_shorter!()