// Generated macro for poll (function)
macro_rules! Depcrate_flockpoll {
() => {
// Module: crate::flock
// Provides: {"poll"}
// Dependencies: {}
fn poll (lockfile : File , done : Arc < AtomicBool >) { loop { thread :: sleep (Duration :: from_millis (500)) ; if done . load (Ordering :: Acquire) || lockfile . set_len (0) . is_err () { return ; } } }
};
}
