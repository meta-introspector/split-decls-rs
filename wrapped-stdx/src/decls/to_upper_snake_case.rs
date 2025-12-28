macro_rules! to_upper_snake_case {
    () => {
        pub fn to_upper_snake_case (s : & str) -> String { to_snake_case (s , char :: to_uppercase) }
    };
}

to_upper_snake_case!()