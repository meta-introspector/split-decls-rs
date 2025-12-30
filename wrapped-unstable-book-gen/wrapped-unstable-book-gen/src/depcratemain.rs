// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let library_path_str = env :: args_os () . nth (1) . expect ("library/ path required") ; let compiler_path_str = env :: args_os () . nth (2) . expect ("compiler/ path required") ; let src_path_str = env :: args_os () . nth (3) . expect ("src/ path required") ; let dest_path_str = env :: args_os () . nth (4) . expect ("destination path required") ; let library_path = Path :: new (& library_path_str) ; let compiler_path = Path :: new (& compiler_path_str) ; let src_path = Path :: new (& src_path_str) ; let dest_path = Path :: new (& dest_path_str) ; let lang_features = collect_lang_features (compiler_path , & mut false) ; let lib_features = collect_lib_features (library_path) . into_iter () . filter (| & (ref name , _) | ! lang_features . contains_key (name)) . collect () ; let env_vars = collect_env_vars (compiler_path) ; let doc_src_path = src_path . join (PATH_STR) ; t ! (fs :: create_dir_all (& dest_path)) ; generate_feature_files (& doc_src_path . join (LANG_FEATURES_DIR) , & dest_path . join (LANG_FEATURES_DIR) , & lang_features ,) ; generate_feature_files (& doc_src_path . join (LIB_FEATURES_DIR) , & dest_path . join (LIB_FEATURES_DIR) , & lib_features ,) ; generate_env_files (& doc_src_path . join (ENV_VARS_DIR) , & dest_path . join (ENV_VARS_DIR) , & env_vars) ; copy_recursive (& doc_src_path , & dest_path) ; generate_summary (& dest_path , & lang_features , & lib_features) ; }
};
}
