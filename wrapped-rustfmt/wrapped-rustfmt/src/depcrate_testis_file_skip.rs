// Generated macro for is_file_skip (function)
macro_rules! Depcrate_testis_file_skip {
() => {
// Module: crate::test
// Provides: {"is_file_skip"}
// Dependencies: {}
fn is_file_skip (path : & Path) -> bool { FILE_SKIP_LIST . iter () . any (| file_path | is_subpath (path , file_path)) }
};
}
