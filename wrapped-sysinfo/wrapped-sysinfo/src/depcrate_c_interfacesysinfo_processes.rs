// Generated macro for sysinfo_processes (function)
macro_rules! Depcrate_c_interfacesysinfo_processes {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_processes"}
// Dependencies: {}
# [doc = " Equivalent of [`System::processes()`][crate::System#method.processes]. Returns an"] # [doc = " array ended by a null pointer. Must be freed."] # [doc = ""] # [doc = " # ⚠\u{fe0f} WARNING ⚠\u{fe0f}"] # [doc = ""] # [doc = " While having this method returned processes, you should *never* call any refresh method!"] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_processes (system : CSystem , fn_pointer : Option < ProcessLoop > , data : * mut c_void ,) -> size_t { assert ! (! system . is_null ()) ; if let Some (fn_pointer) = fn_pointer { unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let len = { let entries = system . processes () ; for (pid , process) in entries { if ! fn_pointer (pid . 0 as _ , process as * const Process as CProcess , data) { break ; } } entries . len () as size_t } ; let _ = Box :: into_raw (system) ; len } } else { 0 } }
};
}
