// Generated macro for unstable_book_path (function)
macro_rules! Depcrate_unstable_bookunstable_book_path {
() => {
// Module: crate::unstable_book
// Provides: {"unstable_book_path"}
// Dependencies: {}
# [doc = " Builds the path to the Unstable Book source directory from the Rust 'src' directory."] pub fn unstable_book_path (base_src_path : & Path) -> PathBuf { base_src_path . join (PATH_STR) }
};
}
