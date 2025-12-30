// Generated macro for should_skip_dir (function)
macro_rules! Depcrate_auto_workspace_generatorshould_skip_dir {
() => {
// Module: crate::auto_workspace_generator
// Provides: {"should_skip_dir"}
// Dependencies: {}
fn should_skip_dir (path : & Path) -> bool { let name = path . file_name () . unwrap () . to_string_lossy () ; matches ! (name . as_ref () , "target" | ".git" | "node_modules") }
};
}
