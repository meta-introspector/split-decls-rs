macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_empty {
    () => {
        deps!();
        # [test] fn str_normalize_empty () { let input = "" ; let pattern = "" ; let expected = "" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into_data () , & pattern . into_data ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_empty!();