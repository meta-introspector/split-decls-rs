// Generated macro for update_root (function)
macro_rules! Depcrate_windows_processupdate_root {
() => {
// Module: crate::windows::process
// Provides: {"update_root"}
// Dependencies: {}
fn update_root (refresh_kind : ProcessRefreshKind , cwd : & Path , root : & mut Option < PathBuf >) { if ! refresh_kind . root () . needs_update (| | root . is_none ()) { return ; } if cwd . has_root () { let mut ancestors = cwd . ancestors () . peekable () ; while let Some (path) = ancestors . next () { if ancestors . peek () . is_none () { * root = Some (path . into ()) ; return ; } } } else { * root = None ; } }
};
}
