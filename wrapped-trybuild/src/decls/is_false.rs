macro_rules! is_false {
    () => {
        fn is_false (boolean : & bool) -> bool { ! * boolean }
    };
}

is_false!()