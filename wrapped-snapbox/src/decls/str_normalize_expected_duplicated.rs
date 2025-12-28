macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_expected_duplicated {
    () => {
        deps!();
        # [test] fn str_normalize_expected_duplicated () { let input = "1
2
3
" ; let pattern = "1
2
2
3
" ; let expected = "1
2
3
" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into_data () , & pattern . into_data ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_expected_duplicated!();