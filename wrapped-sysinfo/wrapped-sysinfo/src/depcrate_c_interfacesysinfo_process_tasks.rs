// Generated macro for sysinfo_process_tasks (function)
macro_rules! Depcrate_c_interfacesysinfo_process_tasks {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_tasks"}
// Dependencies: {}
# [doc = " Equivalent of iterating over [`Process::tasks()`][crate::Process#method.tasks]."] # [doc = ""] # [doc = " # ⚠\u{fe0f} WARNING ⚠\u{fe0f}"] # [doc = ""] # [doc = " While having this method processes, you should *never* call any refresh method!"] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_tasks (process : CProcess , fn_pointer : Option < ProcessPidLoop > , data : * mut c_void ,) -> size_t { assert ! (! process . is_null ()) ; if let Some (fn_pointer) = fn_pointer { unsafe { let process = process as * const Process ; if let Some (tasks) = (* process) . tasks () { for pid in tasks { if ! fn_pointer (pid . 0 as _ , data) { break ; } } tasks . len () as size_t } else { 0 } } } else { 0 } }
};
}
