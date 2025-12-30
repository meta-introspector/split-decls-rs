// Generated macro for get_disks_mapping (function)
macro_rules! Depcrate_unix_freebsd_diskget_disks_mapping {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"get_disks_mapping"}
// Dependencies: {}
fn get_disks_mapping () -> HashMap < String , String > { let mut disk_mapping = HashMap :: new () ; let Some (mapping) = get_sys_value_str_by_name (b"kern.geom.conftxt\0") else { return disk_mapping ; } ; let mut last_id = String :: new () ; for line in mapping . lines () { let mut parts = line . split_whitespace () ; let Some (kind) = parts . next () else { continue } ; # [allow (clippy :: collapsible_if)] if kind == "0" { if let Some ("DISK") = parts . next () && let Some (id) = parts . next () { last_id . clear () ; last_id . push_str (id) ; } } else if kind == "2" && ! last_id . is_empty () { if let Some ("LABEL") = parts . next () && let Some (path) = parts . next () { disk_mapping . insert (format ! ("/dev/{path}") , last_id . clone ()) ; } } } disk_mapping }
};
}
