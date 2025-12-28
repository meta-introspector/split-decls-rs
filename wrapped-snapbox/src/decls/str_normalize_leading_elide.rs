macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_leading_elide {
    () => {
        deps!();
        # [test] fn str_normalize_leading_elide () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "...\nGoodbye" ; let expected = "...\nGoodbye" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_leading_elide!()