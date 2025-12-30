// Generated macro for crate_path (function)
macro_rules! Depcrate_gzcrate_path {
() => {
// Module: crate::gz
// Provides: {"crate_path"}
// Dependencies: {}
fn crate_path (file : & str) -> String { path (Path :: new (env ! ("CARGO_MANIFEST_DIR")) , file) }
};
}
