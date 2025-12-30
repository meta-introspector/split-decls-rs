// Generated macro for sysinfo_process_virtual_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_process_virtual_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_virtual_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::virtual_memory()`][crate::Process#method.virtual_memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_virtual_memory (process : CProcess) -> size_t { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { (* process) . virtual_memory () as usize } }
};
}
