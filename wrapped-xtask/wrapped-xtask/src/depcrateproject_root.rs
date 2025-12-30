// Generated macro for project_root (function)
macro_rules! Depcrateproject_root {
() => {
// Module: crate
// Provides: {"project_root"}
// Dependencies: {}
fn project_root () -> PathBuf { Path :: new (& env ! ("CARGO_MANIFEST_DIR")) . ancestors () . nth (1) . unwrap () . to_path_buf () }
};
}
