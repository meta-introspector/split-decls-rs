macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_post_elide_diverge {
    () => {
        deps!();
        # [test] fn str_normalize_post_elide_diverge () { let input = "Hello\nSun\nAnd\nWorld" ; let pattern = "Hello\n...\nMoon" ; let expected = "Hello\n...\n" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_post_elide_diverge!();