macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_elide_delimited_with_sub {
    () => {
        deps!();
        # [test] fn str_normalize_elide_delimited_with_sub () { let input = "Hello World\nHow are you?\nGoodbye World" ; let pattern = "Hello [..]\n...\nGoodbye [..]" ; let expected = "Hello [..]\n...\nGoodbye [..]" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_elide_delimited_with_sub!()