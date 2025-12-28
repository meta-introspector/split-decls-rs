macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Data!();
        DataInner!();
    };
}

macro_rules! json_normalize_glob_for_string {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_for_string () { let exp = json ! ({ "name" : "{...}" }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "name" : "JohnDoe" }) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
    };
}

json_normalize_glob_for_string!()