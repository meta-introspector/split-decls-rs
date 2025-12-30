// Generated macro for sysinfo_process_root_directory (function)
macro_rules! Depcrate_c_interfacesysinfo_process_root_directory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_root_directory"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::root()`][crate::Process#method.root]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_root_directory (process : CProcess) -> RString { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { if let Some (p) = (* process) . root () . and_then (| root | root . to_str ()) { if let Ok (c) = CString :: new (p) { return c . into_raw () as _ ; } } std :: ptr :: null () } }
};
}
