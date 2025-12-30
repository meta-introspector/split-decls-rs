// Generated macro for lint_docs_path (function)
macro_rules! Depcratelint_docs_path {
() => {
// Module: crate
// Provides: {"lint_docs_path"}
// Dependencies: {}
fn lint_docs_path () -> PathBuf { let pkg_root = env ! ("CARGO_MANIFEST_DIR") ; let ws_root = PathBuf :: from (format ! ("{pkg_root}/../..")) ; let path = { let path = ws_root . join ("src/doc/src/reference/lints.md") ; path . canonicalize () . unwrap_or (path) } ; path }
};
}
