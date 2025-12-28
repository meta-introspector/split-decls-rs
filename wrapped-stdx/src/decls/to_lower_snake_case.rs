macro_rules! to_lower_snake_case {
    () => {
        pub fn to_lower_snake_case (s : & str) -> String { to_snake_case (s , char :: to_lowercase) }
    };
}

to_lower_snake_case!()