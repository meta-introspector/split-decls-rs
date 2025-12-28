macro_rules! strip_trailing_slash {
    () => {
        pub fn strip_trailing_slash (path : & std :: path :: Path) -> & std :: path :: Path { path . components () . as_path () }
    };
}

strip_trailing_slash!();