// Generated macro for procctl (function)
macro_rules! Depcrate_process_procctlprocctl {
() => {
// Module: crate::process::procctl
// Provides: {"procctl"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn procctl (option : c_int , process : ProcSelector , data : * mut c_void ,) -> io :: Result < () > { let (idtype , id) = proc_selector_to_raw (process) ; syscalls :: procctl (idtype as c_uint , id as RawId , option , data) }
};
}
