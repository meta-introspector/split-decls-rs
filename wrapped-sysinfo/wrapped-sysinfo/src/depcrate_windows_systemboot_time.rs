// Generated macro for boot_time (function)
macro_rules! Depcrate_windows_systemboot_time {
() => {
// Module: crate::windows::system
// Provides: {"boot_time"}
// Dependencies: {}
# [doc = " Calculates system boot time in seconds with improved precision."] # [doc = " Uses nanoseconds throughout to avoid rounding errors in uptime calculation,"] # [doc = " converting to seconds only at the end for stable results. Result is capped"] # [doc = " within u64 limits to handle edge cases."] unsafe fn boot_time () -> u64 { match SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) { Ok (n) => { let system_time_ns = n . as_nanos () ; let tick_count_ns = unsafe { GetTickCount64 () } as u128 * 1_000_000 ; let boot_time_sec = system_time_ns . saturating_sub (tick_count_ns) / 1_000_000_000 ; boot_time_sec . try_into () . unwrap_or (u64 :: MAX) } Err (_e) => { sysinfo_debug ! ("Failed to compute boot time: {:?}" , _e) ; 0 } } }
};
}
