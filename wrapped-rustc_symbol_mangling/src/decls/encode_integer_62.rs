macro_rules! encode_integer_62 {
    () => {
        pub (crate) fn encode_integer_62 (x : u64) -> String { let mut output = String :: new () ; push_integer_62 (x , & mut output) ; output }
    };
}

encode_integer_62!();