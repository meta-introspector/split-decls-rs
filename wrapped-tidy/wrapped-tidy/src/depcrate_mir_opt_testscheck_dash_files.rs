// Generated macro for check_dash_files (function)
macro_rules! Depcrate_mir_opt_testscheck_dash_files {
() => {
// Module: crate::mir_opt_tests
// Provides: {"check_dash_files"}
// Dependencies: {}
fn check_dash_files (path : & Path , bless : bool , bad : & mut bool) { for file in walkdir :: WalkDir :: new (path . join ("mir-opt")) . into_iter () . filter_map (Result :: ok) . filter (| e | e . file_type () . is_file ()) { let path = file . path () ; if path . extension () == Some ("rs" . as_ref ()) && let Some (name) = path . file_name () . and_then (| s | s . to_str ()) && name . contains ('-') { if ! bless { tidy_error ! (bad , "mir-opt test files should not have dashes in them: {}" , path . display ()) ; } else { let new_name = name . replace ('-' , "_") ; let mut new_path = path . to_owned () ; new_path . set_file_name (new_name) ; let _ = std :: fs :: rename (path , new_path) ; } } } }
};
}
