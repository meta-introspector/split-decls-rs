macro_rules! is_dec_integer_digit {
    () => {
        fn is_dec_integer_digit (b : u8) -> bool { (b'0' ..= b'9') . contains_token (b) }
    };
}

is_dec_integer_digit!();