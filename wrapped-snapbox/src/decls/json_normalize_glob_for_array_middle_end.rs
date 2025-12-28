macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Data!();
        DataInner!();
    };
}

macro_rules! json_normalize_glob_for_array_middle_end {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_for_array_middle_end () { let exp = json ! ([{ "name" : "one" , "nickname" : "1" , } , "{...}" , { "name" : "three" , "nickname" : "3" , } , "{...}"]) ; let expected = Data :: json (exp) ; let actual = json ! ([{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "three" , "nickname" : "3" , } , { "name" : "four" , "nickname" : "4" , } , { "name" : "five" , "nickname" : "5" , }]) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
    };
}

json_normalize_glob_for_array_middle_end!()