// Generated macro for generate_env_files (function)
macro_rules! Depcrategenerate_env_files {
() => {
// Module: crate
// Provides: {"generate_env_files"}
// Dependencies: {}
fn generate_env_files (src : & Path , out : & Path , env_vars : & BTreeSet < String >) { let env_var_file_names = collect_unstable_book_section_file_names (src) ; t ! (fs :: create_dir_all (& out)) ; for env_var in env_vars - & env_var_file_names { let file_name = format ! ("{env_var}.md") ; let out_file_path = out . join (& file_name) ; generate_stub_env_var (& out_file_path , & env_var) ; } }
};
}
