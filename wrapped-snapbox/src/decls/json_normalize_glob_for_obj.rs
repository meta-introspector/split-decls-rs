macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Data!();
        DataInner!();
    };
}

macro_rules! json_normalize_glob_for_obj {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_for_obj () { let exp = json ! ({ "people" : "{...}" }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : { "name" : "JohnDoe" , "nickname" : "John" , } }) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
    };
}

json_normalize_glob_for_obj!();