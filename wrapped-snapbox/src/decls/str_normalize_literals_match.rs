macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_literals_match {
    () => {
        deps!();
        # [test] fn str_normalize_literals_match () { let input = "Hello\nWorld" ; let pattern = "Hello\nWorld" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_literals_match!()