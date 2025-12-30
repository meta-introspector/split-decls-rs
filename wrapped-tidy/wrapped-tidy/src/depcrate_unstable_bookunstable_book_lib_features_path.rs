// Generated macro for unstable_book_lib_features_path (function)
macro_rules! Depcrate_unstable_bookunstable_book_lib_features_path {
() => {
// Module: crate::unstable_book
// Provides: {"unstable_book_lib_features_path"}
// Dependencies: {}
# [doc = " Builds the path to the directory where the features are documented within the Unstable Book"] # [doc = " source directory."] pub fn unstable_book_lib_features_path (base_src_path : & Path) -> PathBuf { unstable_book_path (base_src_path) . join (LIB_FEATURES_DIR) }
};
}
