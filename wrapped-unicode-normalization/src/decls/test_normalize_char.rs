macro_rules! test_normalize_char {
    () => {
        # [test] fn test_normalize_char () { assert_eq ! ('\u{2126}' . nfd () . to_string () , "\u{3a9}") }
    };
}

test_normalize_char!();