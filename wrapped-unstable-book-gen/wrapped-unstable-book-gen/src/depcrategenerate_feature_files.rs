// Generated macro for generate_feature_files (function)
macro_rules! Depcrategenerate_feature_files {
() => {
// Module: crate
// Provides: {"generate_feature_files"}
// Dependencies: {}
fn generate_feature_files (src : & Path , out : & Path , features : & Features) { let unstable_features = collect_unstable_feature_names (features) ; let unstable_section_file_names = collect_unstable_book_section_file_names (src) ; t ! (fs :: create_dir_all (& out)) ; for feature_name in & unstable_features - & unstable_section_file_names { let feature_name_underscore = feature_name . replace ('-' , "_") ; let file_name = format ! ("{feature_name}.md") ; let out_file_path = out . join (& file_name) ; let feature = & features [& feature_name_underscore] ; let description = feature . description . as_deref () . unwrap_or_default () ; if let Some (issue) = feature . tracking_issue { generate_stub_issue (& out_file_path , & feature_name_underscore , issue . get () , & description ,) ; } else { generate_stub_no_issue (& out_file_path , & feature_name_underscore , & description) ; } } }
};
}
