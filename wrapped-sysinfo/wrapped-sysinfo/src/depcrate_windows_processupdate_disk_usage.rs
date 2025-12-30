// Generated macro for update_disk_usage (function)
macro_rules! Depcrate_windows_processupdate_disk_usage {
() => {
// Module: crate::windows::process
// Provides: {"update_disk_usage"}
// Dependencies: {}
pub (crate) fn update_disk_usage (p : & mut ProcessInner) { let mut counters = MaybeUninit :: < IO_COUNTERS > :: uninit () ; if let Some (handle) = p . get_handle () { unsafe { if GetProcessIoCounters (handle , counters . as_mut_ptr ()) . is_err () { sysinfo_debug ! ("GetProcessIoCounters call failed on process {}" , p . pid ()) ; } else { let counters = counters . assume_init () ; p . old_read_bytes = p . read_bytes ; p . old_written_bytes = p . written_bytes ; p . read_bytes = counters . ReadTransferCount ; p . written_bytes = counters . WriteTransferCount ; } } } }
};
}
