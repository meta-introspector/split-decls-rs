// Generated macro for prctl_get_at_arg2_optional (function)
macro_rules! Depcrate_prctlprctl_get_at_arg2_optional {
() => {
// Module: crate::prctl
// Provides: {"prctl_get_at_arg2_optional"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn prctl_get_at_arg2_optional < P > (option : i32) -> io :: Result < P > { let mut value : MaybeUninit < P > = MaybeUninit :: uninit () ; prctl_2args (option , value . as_mut_ptr () . cast ()) ? ; Ok (value . assume_init ()) }
};
}
