macro_rules! test_is_combining_mark_ascii {
    () => {
        # [test] fn test_is_combining_mark_ascii () { for cp in 0 .. 0x7f { assert ! (! is_combining_mark (char :: from_u32 (cp) . unwrap ())) ; } }
    };
}

test_is_combining_mark_ascii!()