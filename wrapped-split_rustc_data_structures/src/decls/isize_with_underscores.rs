macro_rules! isize_with_underscores {
    () => {
        # [doc = " Print an `isize` with underscore separators."] pub fn isize_with_underscores (n : isize) -> String { format_with_underscores (format ! ("{n}")) }
    };
}

isize_with_underscores!()