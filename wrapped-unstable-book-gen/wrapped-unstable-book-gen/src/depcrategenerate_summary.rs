// Generated macro for generate_summary (function)
macro_rules! Depcrategenerate_summary {
() => {
// Module: crate
// Provides: {"generate_summary"}
// Dependencies: {}
fn generate_summary (path : & Path , lang_features : & Features , lib_features : & Features) { let compiler_flags = collect_unstable_book_section_file_names (& path . join ("src/compiler-flags")) ; let compiler_env_vars = collect_unstable_book_section_file_names (& path . join ("src/compiler-environment-variables")) ; let compiler_flags_str = set_to_summary_str (& compiler_flags , "compiler-flags") ; let compiler_env_vars_str = set_to_summary_str (& compiler_env_vars , "compiler-environment-variables") ; let unstable_lang_features = collect_unstable_feature_names (& lang_features) ; let unstable_lib_features = collect_unstable_feature_names (& lib_features) ; let lang_features_str = set_to_summary_str (& unstable_lang_features , "language-features") ; let lib_features_str = set_to_summary_str (& unstable_lib_features , "library-features") ; let summary_path = path . join ("src/SUMMARY.md") ; let content = format ! (include_str ! ("SUMMARY.md") , compiler_env_vars = compiler_env_vars_str , compiler_flags = compiler_flags_str , language_features = lang_features_str , library_features = lib_features_str) ; t ! (write (& summary_path , content) , summary_path) ; }
};
}
