// Generated macro for target_dir (function)
macro_rules! Depcratetarget_dir {
() => {
// Module: crate
// Provides: {"target_dir"}
// Dependencies: {}
pub fn target_dir () -> Utf8PathBuf { match std :: env :: var ("CARGO_TARGET_DIR") { Ok (target) => Utf8PathBuf :: from (target) , Err (_) => project_root () . join ("target") , } }
};
}
