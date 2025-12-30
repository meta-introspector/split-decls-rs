// Generated macro for refresh_user_group_ids (function)
macro_rules! Depcrate_unix_linux_processrefresh_user_group_ids {
() => {
// Module: crate::unix::linux::process
// Provides: {"refresh_user_group_ids"}
// Dependencies: {}
fn refresh_user_group_ids (p : & mut ProcessInner , path : & mut PathHandler , refresh_kind : ProcessRefreshKind ,) { if ! refresh_kind . user () . needs_update (| | p . user_id . is_none ()) { return ; } if let Some (((user_id , effective_user_id) , (group_id , effective_group_id))) = get_uid_and_gid (path . replace_and_join ("status")) { p . user_id = Some (Uid (user_id)) ; p . effective_user_id = Some (Uid (effective_user_id)) ; p . group_id = Some (Gid (group_id)) ; p . effective_group_id = Some (Gid (effective_group_id)) ; } }
};
}
