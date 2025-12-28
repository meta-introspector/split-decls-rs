macro_rules! is_ascii_lowercase_hex {
    () => {
        fn is_ascii_lowercase_hex (s : & str) -> bool { s . bytes () . all (| b | matches ! (b , b'0' ..= b'9' | b'a' ..= b'f')) }
    };
}

is_ascii_lowercase_hex!();