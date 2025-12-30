// Generated macro for prctl_get_at_arg2 (function)
macro_rules! Depcrate_prctlprctl_get_at_arg2 {
() => {
// Module: crate::prctl
// Provides: {"prctl_get_at_arg2"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn prctl_get_at_arg2 < P , T > (option : i32) -> io :: Result < T > where P : Default , T : TryFrom < P , Error = io :: Errno > , { let mut value : P = Default :: default () ; prctl_2args (option , as_mut_ptr (& mut value) . cast ()) ? ; TryFrom :: try_from (value) }
};
}
