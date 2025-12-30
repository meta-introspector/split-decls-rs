// Generated macro for old_get_memory (function)
macro_rules! Depcrate_unix_linux_processold_get_memory {
() => {
// Module: crate::unix::linux::process
// Provides: {"old_get_memory"}
// Dependencies: {}
fn old_get_memory (entry : & mut ProcessInner , str_parts : & [& str] , info : & SystemInfo) { entry . memory = u64 :: from_str (str_parts [ProcIndex :: ResidentSetSize as usize]) . unwrap_or (0) . saturating_mul (info . page_size_b) ; entry . virtual_memory = u64 :: from_str (str_parts [ProcIndex :: VirtualSize as usize]) . unwrap_or (0) ; }
};
}
