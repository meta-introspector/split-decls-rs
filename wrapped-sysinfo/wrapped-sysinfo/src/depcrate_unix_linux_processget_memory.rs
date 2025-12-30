// Generated macro for get_memory (function)
macro_rules! Depcrate_unix_linux_processget_memory {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_memory"}
// Dependencies: {}
fn get_memory (path : & Path , entry : & mut ProcessInner , info : & SystemInfo) -> bool { let mut file = match File :: open (path) { Ok (f) => f , Err (_e) => { sysinfo_debug ! ("Using old memory information (failed to open {:?}: {_e:?})" , path) ; return false ; } } ; let mut buf = Vec :: new () ; if let Err (_e) = file . read_to_end (& mut buf) { sysinfo_debug ! ("Using old memory information (failed to read {:?}: {_e:?})" , path) ; return false ; } let mut parts = buf . split (| c | * c == b' ') ; entry . virtual_memory = parts . next () . map (slice_to_nb) . unwrap_or (0) . saturating_mul (info . page_size_b) ; entry . memory = parts . next () . map (slice_to_nb) . unwrap_or (0) . saturating_mul (info . page_size_b) ; true }
};
}
