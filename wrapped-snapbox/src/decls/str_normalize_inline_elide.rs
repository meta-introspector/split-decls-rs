macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! str_normalize_inline_elide {
    () => {
        deps!();
        # [test] fn str_normalize_inline_elide () { let input = "Hello\nWorld\nGoodbye\nSir" ; let pattern = "Hello\nW[..]d\nGoodbye\nSir" ; let expected = "Hello\nW[..]d\nGoodbye\nSir" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
    };
}

str_normalize_inline_elide!();