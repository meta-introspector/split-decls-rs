macro_rules! deps {
    () => {
        DataInner!();
        Data!();
        NormalizeToExpected!();
    };
}

macro_rules! json_normalize_glob_array_start {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_array_start () { let exp = json ! ({ "people" : ["{...}" , { "name" : "three" , "nickname" : "3" , }] }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : [{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "three" , "nickname" : "3" , }] }) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
    };
}

json_normalize_glob_array_start!()