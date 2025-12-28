macro_rules! hide_trailing_numbers {
    () => {
        fn hide_trailing_numbers (line : & mut String) { for _ in 0 .. 2 { let digits = line . bytes () . rev () . take_while (u8 :: is_ascii_digit) . count () ; if digits == 0 || ! line [.. line . len () - digits] . ends_with (':') { return ; } line . truncate (line . len () - digits - 1) ; } }
    };
}

hide_trailing_numbers!();