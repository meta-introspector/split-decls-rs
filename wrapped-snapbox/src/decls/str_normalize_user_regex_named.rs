macro_rules! deps {
    () => {
        Redactions!();
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_user_regex_named {
    () => {
        deps!();
        # [test] # [cfg (feature = "regex")] fn str_normalize_user_regex_named () { let input = "Hello world!" ; let pattern = "Hello [OBJECT]!" ; let mut sub = Redactions :: new () ; sub . insert ("[OBJECT]" , regex :: Regex :: new ("(?<redacted>world)!") . unwrap () ,) . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
    };
}

str_normalize_user_regex_named!();