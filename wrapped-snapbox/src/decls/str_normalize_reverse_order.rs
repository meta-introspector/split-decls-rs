macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_reverse_order {
    () => {
        deps!();
        # [test] fn str_normalize_reverse_order () { let input = "1
2
3
" ; let pattern = "3
2
1
" ; let expected = "3
2
1
" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into_data () , & pattern . into_data ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_reverse_order!()