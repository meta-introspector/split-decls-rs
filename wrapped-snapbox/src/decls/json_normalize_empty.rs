macro_rules! deps {
    () => {
        Data!();
        NormalizeToExpected!();
    };
}

macro_rules! json_normalize_empty {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_empty () { let input = json ! ([]) ; let pattern = json ! ([]) ; let expected = json ! ([]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
    };
}

json_normalize_empty!();