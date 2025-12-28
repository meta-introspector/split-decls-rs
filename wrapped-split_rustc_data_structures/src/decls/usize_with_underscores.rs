macro_rules! usize_with_underscores {
    () => {
        # [doc = " Print a `usize` with underscore separators."] pub fn usize_with_underscores (n : usize) -> String { format_with_underscores (format ! ("{n}")) }
    };
}

usize_with_underscores!()