// Generated macro for check (function)
macro_rules! Depcrate_unstable_bookcheck {
() => {
// Module: crate::unstable_book
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , features : CollectedFeatures , bad : & mut bool) { let lang_features = features . lang ; let lib_features = features . lib . into_iter () . filter (| (name , _) | ! lang_features . contains_key (name)) . collect :: < Features > () ; let unstable_lib_feature_names = collect_unstable_feature_names (& lib_features) ; let unstable_book_lib_features_section_file_names = collect_unstable_book_lib_features_section_file_names (path) ; let unstable_lang_feature_names = collect_unstable_feature_names (& lang_features) ; let unstable_book_lang_features_section_file_names = collect_unstable_book_lang_features_section_file_names (path) ; for feature_name in & unstable_book_lib_features_section_file_names - & unstable_lib_feature_names { tidy_error ! (bad , "The Unstable Book has a 'library feature' section '{}' which doesn't \
                         correspond to an unstable library feature" , feature_name) ; maybe_suggest_dashes (& unstable_lib_feature_names , & feature_name , bad) ; } for feature_name in & unstable_book_lang_features_section_file_names - & unstable_lang_feature_names { tidy_error ! (bad , "The Unstable Book has a 'language feature' section '{}' which doesn't \
                     correspond to an unstable language feature" , feature_name) ; maybe_suggest_dashes (& unstable_lang_feature_names , & feature_name , bad) ; } }
};
}
