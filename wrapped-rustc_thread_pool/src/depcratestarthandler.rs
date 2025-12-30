// Generated macro for StartHandler (type)
macro_rules! DepcrateStartHandler {
() => {
// Module: crate
// Provides: {"StartHandler"}
// Dependencies: {}
# [doc = " The type for a closure that gets invoked when a thread starts. The"] # [doc = " closure is passed the index of the thread on which it is invoked."] # [doc = " Note that this same closure may be invoked multiple times in parallel."] type StartHandler = dyn Fn (usize) + Send + Sync ;
};
}
