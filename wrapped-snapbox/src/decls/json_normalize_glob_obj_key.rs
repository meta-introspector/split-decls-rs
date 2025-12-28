macro_rules! deps {
    () => {
        Data!();
        NormalizeToExpected!();
    };
}

macro_rules! json_normalize_glob_obj_key {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_glob_obj_key () { let expected = json ! ({ "a" : "value-a" , "c" : "value-c" , "..." : "{...}" , }) ; let expected = Data :: json (expected) ; let actual = json ! ({ "a" : "value-a" , "b" : "value-b" , "c" : "value-c" , }) ; let actual = Data :: json (actual) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (actual , & expected) ; let expected_actual = json ! ({ "a" : "value-a" , "c" : "value-c" , "..." : "{...}" , }) ; let expected_actual = Data :: json (expected_actual) ; assert_eq ! (actual , expected_actual) ; }
    };
}

json_normalize_glob_obj_key!()