// Generated macro for sleep (function)
macro_rules! Depcratesleep {
() => {
// Module: crate
// Provides: {"sleep"}
// Dependencies: {}
# [doc = " Suspends the execution of the current thread until the time-out interval elapses."] pub fn sleep (milliseconds : u32) { unsafe { Sleep (milliseconds) ; } }
};
}
