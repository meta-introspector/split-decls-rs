// Generated macro for tests (module)
macro_rules! Depcrate_flagtests {
() => {
// Module: crate::flag
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: sync :: atomic ; use std :: time :: { Duration , Instant } ; use super :: * ; use crate :: consts :: signal :: * ; fn self_signal () { # [cfg (not (windows))] const SIG : c_int = SIGUSR1 ; # [cfg (windows)] const SIG : c_int = SIGTERM ; crate :: low_level :: raise (SIG) . unwrap () ; } fn wait_flag (flag : & AtomicBool) -> bool { let start = Instant :: now () ; while ! flag . load (Ordering :: Relaxed) { # [allow (deprecated)] atomic :: spin_loop_hint () ; if Instant :: now () - start > Duration :: from_secs (1) { return false ; } } true } # [test] fn register_unregister () { let flag = Arc :: new (AtomicBool :: new (false)) ; # [cfg (not (windows))] let signal = register (SIGUSR1 , Arc :: clone (& flag)) . unwrap () ; # [cfg (windows)] let signal = register (crate :: SIGTERM , Arc :: clone (& flag)) . unwrap () ; self_signal () ; assert ! (wait_flag (& flag)) ; assert ! (crate :: low_level :: unregister (signal)) ; flag . store (false , Ordering :: Relaxed) ; self_signal () ; assert ! (! wait_flag (& flag)) ; assert_eq ! (1 , Arc :: strong_count (& flag)) ; } }
};
}
