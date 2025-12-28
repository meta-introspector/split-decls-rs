macro_rules! ParseError {
    () => {
        pub struct ParseError < 'a > { unknown : & 'a str , }
    };
}

ParseError!();