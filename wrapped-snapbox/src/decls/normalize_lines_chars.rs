macro_rules! normalize_lines_chars {
    () => {
        fn normalize_lines_chars (data : impl Iterator < Item = char >) -> impl Iterator < Item = char > { normalize_line_endings :: normalized (data) }
    };
}

normalize_lines_chars!();