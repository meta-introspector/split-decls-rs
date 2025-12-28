macro_rules! hide_leading_numbers {
    () => {
        fn hide_leading_numbers (line : & mut String) { let n = line . bytes () . take_while (| b : & u8 | * b == b' ' || b . is_ascii_digit ()) . count () ; for i in 0 .. n { line . replace_range (i .. i + 1 , " ") ; } }
    };
}

hide_leading_numbers!()