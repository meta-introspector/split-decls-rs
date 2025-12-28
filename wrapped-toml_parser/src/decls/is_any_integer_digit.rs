macro_rules! is_any_integer_digit {
    () => {
        fn is_any_integer_digit (b : u8) -> bool { (b'0' ..= b'9' , b'a' ..= b'f' , b'A' ..= b'F') . contains_token (b) }
    };
}

is_any_integer_digit!();