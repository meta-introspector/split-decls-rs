// Generated macro for disk_stats (function)
macro_rules! Depcrate_unix_linux_diskdisk_stats {
() => {
// Module: crate::unix::linux::disk
// Provides: {"disk_stats"}
// Dependencies: {}
fn disk_stats (refresh_kind : & DiskRefreshKind) -> HashMap < String , DiskStat > { if refresh_kind . io_usage () { let path = "/proc/diskstats" ; match fs :: read_to_string (path) { Ok (content) => disk_stats_inner (& content) , Err (_error) => { sysinfo_debug ! ("failed to read {path:?}: {_error:?}") ; HashMap :: new () } } } else { Default :: default () } }
};
}
