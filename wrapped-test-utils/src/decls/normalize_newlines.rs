macro_rules! normalize_newlines {
    () => {
        fn normalize_newlines (s : & str) -> String { s . replace ("\r\n" , "\n") }
    };
}

normalize_newlines!()