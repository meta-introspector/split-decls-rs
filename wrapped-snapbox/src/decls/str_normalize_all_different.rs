macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_all_different {
    () => {
        deps!();
        # [test] fn str_normalize_all_different () { let input = "Hello\nWorld" ; let pattern = "Goodbye\nMoon" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_all_different!();