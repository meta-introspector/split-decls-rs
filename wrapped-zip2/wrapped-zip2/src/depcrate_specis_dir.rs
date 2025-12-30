// Generated macro for is_dir (function)
macro_rules! Depcrate_specis_dir {
() => {
// Module: crate::spec
// Provides: {"is_dir"}
// Dependencies: {}
pub (crate) fn is_dir (filename : & str) -> bool { filename . chars () . next_back () . is_some_and (| c | c == '/' || c == '\\') }
};
}
