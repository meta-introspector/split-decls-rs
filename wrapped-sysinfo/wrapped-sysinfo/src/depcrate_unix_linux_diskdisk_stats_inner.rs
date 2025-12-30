// Generated macro for disk_stats_inner (function)
macro_rules! Depcrate_unix_linux_diskdisk_stats_inner {
() => {
// Module: crate::unix::linux::disk
// Provides: {"disk_stats_inner"}
// Dependencies: {}
fn disk_stats_inner (content : & str) -> HashMap < String , DiskStat > { let mut data = HashMap :: new () ; for line in content . lines () { let line = line . trim () ; if line . is_empty () { continue ; } if let Some ((name , stats)) = DiskStat :: new_from_line (line) { data . insert (name , stats) ; } } data }
};
}
