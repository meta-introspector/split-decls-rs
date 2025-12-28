macro_rules! normalize_paths_chars {
    () => {
        fn normalize_paths_chars (data : impl Iterator < Item = char >) -> impl Iterator < Item = char > { data . map (| c | if c == '\\' { '/' } else { c }) }
    };
}

normalize_paths_chars!();