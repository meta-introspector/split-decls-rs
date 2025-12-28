macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Data!();
        DataInner!();
    };
}

macro_rules! json_normalize_glob_for_array_mismatch {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_for_array_mismatch () { let exp = json ! ([{ "name" : "one" , "nickname" : "1" , } , { "name" : "three" , "nickname" : "3" , } , "{...}"]) ; let expected = Data :: json (exp) ; let actual = json ! ([{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "four" , "nickname" : "4" , } , { "name" : "five" , "nickname" : "5" , }]) ; let expected_actual = json ! ([{ "name" : "one" , "nickname" : "1" , } , "{...}"]) ; let actual_normalized = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual . clone ()) , & expected) ; if let DataInner :: Json (act) = actual_normalized . inner { assert_eq ! (act , expected_actual) ; } }
    };
}

json_normalize_glob_for_array_mismatch!();