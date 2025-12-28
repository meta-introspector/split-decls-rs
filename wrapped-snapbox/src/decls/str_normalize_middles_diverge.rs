macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_middles_diverge {
    () => {
        deps!();
        # [test] fn str_normalize_middles_diverge () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "Hello\nMoon\nGoodbye" ; let expected = "Hello\nWorld\nGoodbye" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_middles_diverge!()