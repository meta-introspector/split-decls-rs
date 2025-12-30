// Generated macro for changed (function)
macro_rules! Depcrate_xtaskchanged {
() => {
// Module: crate::xtask
// Provides: {"changed"}
// Dependencies: {}
# [doc = " Lists all changed workspace members between two commits."] fn changed < 'r , 'ws > (ws : & 'ws Workspace < '_ > , repo : & 'r git2 :: Repository , base_commit : & git2 :: Commit < 'r > , head : & git2 :: Commit < 'r > ,) -> CargoResult < HashMap < & 'ws str , & 'ws Package > > { let root_pkg_name = ws . current () ? . name () ; let ws_members = ws . members () . filter (| pkg | pkg . name () != root_pkg_name) . filter (| pkg | pkg . publish () != & Some (vec ! [])) . map (| pkg | { let relative_pkg_root = pkg . root () . strip_prefix (ws . root ()) . unwrap () ; (relative_pkg_root , pkg) }) . collect :: < Vec < _ > > () ; let diff = symmetric_diff (repo , base_commit , head) ? ; let mut changed_members = HashMap :: new () ; for delta in diff . deltas () { let old = delta . old_file () . path () . unwrap () ; let new = delta . new_file () . path () . unwrap () ; for (pkg_root , pkg) in ws_members . iter () { if old . starts_with (pkg_root) || new . starts_with (pkg_root) { changed_members . insert (pkg . name () . as_str () , * pkg) ; break ; } } } tracing :: trace ! ("changed_members: {:?}" , changed_members . keys ()) ; Ok (changed_members) }
};
}
