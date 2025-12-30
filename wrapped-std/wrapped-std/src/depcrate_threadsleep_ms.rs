// Generated macro for sleep_ms (function)
macro_rules! Depcrate_threadsleep_ms {
() => {
// Module: crate::thread
// Provides: {"sleep_ms"}
// Dependencies: {}
# [doc = " Uses [`sleep`]."] # [doc = ""] # [doc = " Puts the current thread to sleep for at least the specified amount of time."] # [doc = ""] # [doc = " The thread may sleep longer than the duration specified due to scheduling"] # [doc = " specifics or platform-dependent functionality. It will never sleep less."] # [doc = ""] # [doc = " This function is blocking, and should not be used in `async` functions."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " On Unix platforms, the underlying syscall may be interrupted by a"] # [doc = " spurious wakeup or signal handler. To ensure the sleep occurs for at least"] # [doc = " the specified duration, this function may invoke that system call multiple"] # [doc = " times."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " // Let's sleep for 2 seconds:"] # [doc = " thread::sleep_ms(2000);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [deprecated (since = "1.6.0" , note = "replaced by `std::thread::sleep`")] pub fn sleep_ms (ms : u32) { sleep (Duration :: from_millis (ms as u64)) }
};
}
