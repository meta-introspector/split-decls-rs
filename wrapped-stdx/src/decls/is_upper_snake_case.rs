macro_rules! is_upper_snake_case {
    () => {
        # [must_use] pub fn is_upper_snake_case (s : & str) -> bool { s . chars () . all (| c | c . is_uppercase () || c == '_' || c . is_numeric ()) }
    };
}

is_upper_snake_case!()