macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_post_diverge_elide {
    () => {
        deps!();
        # [test] fn str_normalize_post_diverge_elide () { let input = "Hello\nWorld\nGoodbye\nSir" ; let pattern = "Hello\nMoon\nGoodbye\n..." ; let expected = "Hello\nGoodbye\n..." ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_post_diverge_elide!()