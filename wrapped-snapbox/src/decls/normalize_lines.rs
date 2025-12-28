macro_rules! normalize_lines {
    () => {
        # [doc = " Normalize line endings"] pub fn normalize_lines (data : & str) -> String { normalize_lines_chars (data . chars ()) . collect () }
    };
}

normalize_lines!();