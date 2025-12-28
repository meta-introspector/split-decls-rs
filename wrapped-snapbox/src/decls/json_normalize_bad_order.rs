macro_rules! deps {
    () => {
        NormalizeToExpected!();
        DataInner!();
        Data!();
    };
}

macro_rules! json_normalize_bad_order {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_bad_order () { let exp = json ! ({ "people" : ["John" , "Jane"] }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : ["Jane" , "John"] }) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_ne ! (exp , act) ; } }
    };
}

json_normalize_bad_order!();