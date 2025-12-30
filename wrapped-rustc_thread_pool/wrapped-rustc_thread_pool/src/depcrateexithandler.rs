// Generated macro for ExitHandler (type)
macro_rules! DepcrateExitHandler {
() => {
// Module: crate
// Provides: {"ExitHandler"}
// Dependencies: {}
# [doc = " The type for a closure that gets invoked when a thread exits. The"] # [doc = " closure is passed the index of the thread on which it is invoked."] # [doc = " Note that this same closure may be invoked multiple times in parallel."] type ExitHandler = dyn Fn (usize) + Send + Sync ;
};
}
