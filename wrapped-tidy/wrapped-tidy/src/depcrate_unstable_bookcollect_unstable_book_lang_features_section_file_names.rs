// Generated macro for collect_unstable_book_lang_features_section_file_names (function)
macro_rules! Depcrate_unstable_bookcollect_unstable_book_lang_features_section_file_names {
() => {
// Module: crate::unstable_book
// Provides: {"collect_unstable_book_lang_features_section_file_names"}
// Dependencies: {}
# [doc = " Retrieves file names of all library feature sections in the Unstable Book with:"] # [doc = ""] # [doc = " * hyphens replaced by underscores,"] # [doc = " * the markdown suffix ('.md') removed."] fn collect_unstable_book_lang_features_section_file_names (base_src_path : & Path ,) -> BTreeSet < String > { collect_unstable_book_section_file_names (& unstable_book_lang_features_path (base_src_path)) }
};
}
