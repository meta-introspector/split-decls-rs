// Generated macro for sysinfo_process_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_process_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::memory()`][crate::Process#method.memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_memory (process : CProcess) -> size_t { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { (* process) . memory () as usize } }
};
}
