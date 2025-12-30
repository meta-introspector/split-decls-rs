// Generated macro for procctl_set (function)
macro_rules! Depcrate_process_procctlprocctl_set {
() => {
// Module: crate::process::procctl
// Provides: {"procctl_set"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn procctl_set < P > (option : c_int , process : ProcSelector , data : & P ,) -> io :: Result < () > { procctl (option , process , (as_ptr (data) as * mut P) . cast ()) }
};
}
