// Generated macro for cpuinfo_is_key (function)
macro_rules! Depcrate_unix_linux_cpucpuinfo_is_key {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"cpuinfo_is_key"}
// Dependencies: {}
# [inline] fn cpuinfo_is_key (line : & str , key : & [u8]) -> bool { let line = line . as_bytes () ; line . len () > key . len () && line [.. key . len ()] . eq_ignore_ascii_case (key) }
};
}
