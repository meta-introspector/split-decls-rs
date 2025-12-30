// Generated macro for refresh_disk_io (function)
macro_rules! Depcrate_unix_freebsd_diskrefresh_disk_io {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"refresh_disk_io"}
// Dependencies: {}
unsafe fn refresh_disk_io < T : GetValues > (disks : & mut [T]) { static GEOM_STATS : OnceLock < Result < () , () > > = OnceLock :: new () ; if GEOM_STATS . get_or_init (| | unsafe { initialize_geom () }) . is_err () { return ; } let snap = unsafe { GeomSnapshot :: new () } ; let Some (mut snap) = snap else { return ; } ; for device in snap . iter () { let device = unsafe { device . devstat . as_ref () } ; let Some (device_name) = c_buf_to_utf8_str (& device . device_name) else { continue ; } ; let dev_stat_name = format ! ("{device_name}{}" , device . unit_number) ; for disk in disks . iter_mut () . filter (| d | d . dev_id () . is_some_and (| id | * id == dev_stat_name)) { disk . update_old () ; * disk . get_read () = device . bytes [DEVSTAT_READ] ; * disk . get_written () = device . bytes [DEVSTAT_WRITE] ; } } }
};
}
