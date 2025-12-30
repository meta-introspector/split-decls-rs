// Generated macro for get_target (function)
macro_rules! Depcrate_testget_target {
() => {
// Module: crate::test
// Provides: {"get_target"}
// Dependencies: {}
fn get_target (file_name : & Path , target : Option < & str >) -> PathBuf { if let Some (n) = file_name . components () . position (| c | c . as_os_str () == "source") { let mut target_file_name = PathBuf :: new () ; for (i , c) in file_name . components () . enumerate () { if i == n { target_file_name . push ("target") ; } else { target_file_name . push (c . as_os_str ()) ; } } if let Some (replace_name) = target { target_file_name . with_file_name (replace_name) } else { target_file_name } } else { file_name . to_owned () } }
};
}
