// Generated macro for DeadlockHandler (type)
macro_rules! DepcrateDeadlockHandler {
() => {
// Module: crate
// Provides: {"DeadlockHandler"}
// Dependencies: {}
# [doc = " The type for a closure that gets invoked when the Rayon thread pool deadlocks"] type DeadlockHandler = dyn Fn () + Send + Sync ;
};
}
