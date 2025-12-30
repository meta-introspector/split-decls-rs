// Generated macro for procctl_get_optional (function)
macro_rules! Depcrate_process_procctlprocctl_get_optional {
() => {
// Module: crate::process::procctl
// Provides: {"procctl_get_optional"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn procctl_get_optional < P > (option : c_int , process : ProcSelector ,) -> io :: Result < P > { let mut value : MaybeUninit < P > = MaybeUninit :: uninit () ; procctl (option , process , value . as_mut_ptr () . cast ()) ? ; Ok (value . assume_init ()) }
};
}
