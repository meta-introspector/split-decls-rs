macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_trailing_elide {
    () => {
        deps!();
        # [test] fn str_normalize_trailing_elide () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "Hello\n..." ; let expected = "Hello\n..." ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_trailing_elide!()