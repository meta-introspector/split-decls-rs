macro_rules! is_lower_hex_digit {
    () => {
        fn is_lower_hex_digit (byte : u8) -> bool { byte >= b'0' && byte <= b'9' || byte >= b'a' && byte <= b'f' }
    };
}

is_lower_hex_digit!();