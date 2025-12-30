// Generated macro for impl_430 (impl)
macro_rules! Depcrate_sigabiimpl_430 {
() => {
// Module: crate::sigabi
// Provides: {"impl_430"}
// Dependencies: {}
impl SigProcControl { pub fn signal_will_ign (& self , sig : usize , is_parent_sigchld : bool) -> bool { let flags = self . actions [sig - 1] . first . load (Ordering :: Relaxed) ; let will_ign = flags & (1 << 63) != 0 ; let sig_specific = flags & (1 << 62) != 0 ; will_ign || (sig == SIGCHLD && is_parent_sigchld && sig_specific) } pub fn signal_will_stop (& self , sig : usize) -> bool { use crate :: flag :: * ; matches ! (sig , SIGTSTP | SIGTTIN | SIGTTOU) && self . actions [sig - 1] . first . load (Ordering :: Relaxed) & (1 << 62) != 0 } }
};
}
