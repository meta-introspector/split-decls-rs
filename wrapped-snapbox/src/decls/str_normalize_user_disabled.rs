macro_rules! deps {
    () => {
        Redactions!();
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_user_disabled {
    () => {
        deps!();
        # [test] fn str_normalize_user_disabled () { let input = "cargo" ; let pattern = "cargo[EXE]" ; let mut sub = Redactions :: new () ; sub . insert ("[EXE]" , "") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
    };
}

str_normalize_user_disabled!();