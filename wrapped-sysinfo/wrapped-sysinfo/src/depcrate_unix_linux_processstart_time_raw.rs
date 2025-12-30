// Generated macro for start_time_raw (function)
macro_rules! Depcrate_unix_linux_processstart_time_raw {
() => {
// Module: crate::unix::linux::process
// Provides: {"start_time_raw"}
// Dependencies: {}
# [inline (always)] fn start_time_raw (parts : & Parts < '_ >) -> u64 { u64 :: from_str (parts . str_parts [ProcIndex :: StartTime as usize]) . unwrap_or (0) }
};
}
