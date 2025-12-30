// Generated macro for update_proc_disk_activity (function)
macro_rules! Depcrate_unix_apple_macos_processupdate_proc_disk_activity {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"update_proc_disk_activity"}
// Dependencies: {}
fn update_proc_disk_activity (p : & mut ProcessInner) { p . old_read_bytes = p . read_bytes ; p . old_written_bytes = p . written_bytes ; let mut pidrusage = MaybeUninit :: < libc :: rusage_info_v2 > :: uninit () ; unsafe { let retval = libc :: proc_pid_rusage (p . pid () . 0 as _ , libc :: RUSAGE_INFO_V2 , pidrusage . as_mut_ptr () as _ ,) ; if retval < 0 { sysinfo_debug ! ("proc_pid_rusage failed: {:?}" , retval) ; } else { let pidrusage = pidrusage . assume_init () ; p . read_bytes = pidrusage . ri_diskio_bytesread ; p . written_bytes = pidrusage . ri_diskio_byteswritten ; } } }
};
}
