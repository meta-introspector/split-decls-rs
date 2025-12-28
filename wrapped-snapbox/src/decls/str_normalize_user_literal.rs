macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Redactions!();
    };
}

macro_rules! str_normalize_user_literal {
    () => {
        deps!();
        # [test] fn str_normalize_user_literal () { let input = "Hello world!" ; let pattern = "Hello [OBJECT]!" ; let mut sub = Redactions :: new () ; sub . insert ("[OBJECT]" , "world") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
    };
}

str_normalize_user_literal!()