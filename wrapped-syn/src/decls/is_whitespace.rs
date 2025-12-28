macro_rules! is_whitespace {
    () => {
        fn is_whitespace (ch : char) -> bool { ch . is_whitespace () || ch == '\u{200e}' || ch == '\u{200f}' }
    };
}

is_whitespace!()