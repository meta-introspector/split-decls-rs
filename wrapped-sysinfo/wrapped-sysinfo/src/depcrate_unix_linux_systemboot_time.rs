// Generated macro for boot_time (function)
macro_rules! Depcrate_unix_linux_systemboot_time {
() => {
// Module: crate::unix::linux::system
// Provides: {"boot_time"}
// Dependencies: {}
fn boot_time () -> u64 { if let Ok (buf) = File :: open ("/proc/stat") . and_then (| mut f | { let mut buf = Vec :: new () ; f . read_to_end (& mut buf) ? ; Ok (buf) }) { let line = buf . split (| c | * c == b'\n') . find (| l | l . starts_with (b"btime")) ; if let Some (line) = line { return line . split (| x | * x == b' ') . filter (| s | ! s . is_empty ()) . nth (1) . map (to_u64) . unwrap_or (0) ; } } unsafe { let mut up : libc :: timespec = std :: mem :: zeroed () ; if libc :: clock_gettime (libc :: CLOCK_BOOTTIME , & mut up) == 0 { up . tv_sec as u64 } else { sysinfo_debug ! ("clock_gettime failed: boot time cannot be retrieve...") ; 0 } } }
};
}
