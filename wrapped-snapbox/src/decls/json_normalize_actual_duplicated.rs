macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Data!();
    };
}

macro_rules! json_normalize_actual_duplicated {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_actual_duplicated () { let input = json ! ([1 , 2 , 2 , 3]) ; let pattern = json ! ([1 , 2 , 3]) ; let expected = json ! ([1 , 2 , 3 , 2]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
    };
}

json_normalize_actual_duplicated!()