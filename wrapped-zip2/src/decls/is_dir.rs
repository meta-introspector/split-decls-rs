macro_rules! is_dir {
    () => {
        pub (crate) fn is_dir (filename : & str) -> bool { filename . chars () . next_back () . is_some_and (| c | c == '/' || c == '\\') }
    };
}

is_dir!();