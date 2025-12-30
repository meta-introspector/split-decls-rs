// Generated macro for Mutex (struct)
macro_rules! Depcrate_loom_std_mutexMutex {
() => {
// Module: crate::loom::std::mutex
// Provides: {"Mutex"}
// Dependencies: {}
# [doc = " Adapter for `std::Mutex` that removes the poisoning aspects"] # [doc = " from its API."] # [derive (Debug)] pub (crate) struct Mutex < T : ? Sized > (sync :: Mutex < T >) ;
};
}
