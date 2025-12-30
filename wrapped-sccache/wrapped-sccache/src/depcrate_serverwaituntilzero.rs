// Generated macro for WaitUntilZero (struct)
macro_rules! Depcrate_serverWaitUntilZero {
() => {
// Module: crate::server
// Provides: {"WaitUntilZero"}
// Dependencies: {}
# [doc = " Helper future which tracks the `ActiveInfo` below. This future will resolve"] # [doc = " once all instances of `ActiveInfo` have been dropped."] struct WaitUntilZero { info : std :: sync :: Weak < std :: sync :: Mutex < Info > > , }
};
}
