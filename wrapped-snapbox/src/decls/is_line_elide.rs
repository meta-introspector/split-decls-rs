macro_rules! is_line_elide {
    () => {
        fn is_line_elide (line : & str) -> bool { line == "...\n" || line == "..." }
    };
}

is_line_elide!();