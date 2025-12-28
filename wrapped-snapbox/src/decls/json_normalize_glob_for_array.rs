macro_rules! deps {
    () => {
        Data!();
        DataInner!();
        NormalizeToExpected!();
    };
}

macro_rules! json_normalize_glob_for_array {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_for_array () { let exp = json ! ({ "people" : "{...}" }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : [{ "name" : "JohnDoe" , "nickname" : "John" , }] }) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
    };
}

json_normalize_glob_for_array!()