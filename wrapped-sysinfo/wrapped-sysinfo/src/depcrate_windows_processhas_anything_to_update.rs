// Generated macro for has_anything_to_update (function)
macro_rules! Depcrate_windows_processhas_anything_to_update {
() => {
// Module: crate::windows::process
// Provides: {"has_anything_to_update"}
// Dependencies: {}
fn has_anything_to_update (process : & ProcessInner , refresh_kind : ProcessRefreshKind) -> bool { refresh_kind . cmd () . needs_update (| | process . cmd . is_empty ()) || refresh_kind . environ () . needs_update (| | process . environ . is_empty ()) || refresh_kind . cwd () . needs_update (| | process . cwd . is_none ()) || refresh_kind . root () . needs_update (| | process . root . is_none ()) }
};
}
