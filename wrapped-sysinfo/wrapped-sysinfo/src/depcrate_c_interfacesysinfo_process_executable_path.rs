// Generated macro for sysinfo_process_executable_path (function)
macro_rules! Depcrate_c_interfacesysinfo_process_executable_path {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_executable_path"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::exe()`][crate::Process#method.exe]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_executable_path (process : CProcess) -> RString { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { if let Some (p) = (* process) . exe () . and_then (| exe | exe . to_str ()) { if let Ok (c) = CString :: new (p) { return c . into_raw () as _ ; } } std :: ptr :: null () } }
};
}
