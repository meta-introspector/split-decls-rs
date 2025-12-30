// Generated macro for Thread (struct)
macro_rules! Depcrate_threadThread {
() => {
// Module: crate::thread
// Provides: {"Thread"}
// Dependencies: {}
# [derive (Clone)] # [stable (feature = "rust1" , since = "1.0.0")] # [doc = " A handle to a thread."] # [doc = ""] # [doc = " Threads are represented via the `Thread` type, which you can get in one of"] # [doc = " two ways:"] # [doc = ""] # [doc = " * By spawning a new thread, e.g., using the [`thread::spawn`][`spawn`]"] # [doc = "   function, and calling [`thread`][`JoinHandle::thread`] on the"] # [doc = "   [`JoinHandle`]."] # [doc = " * By requesting the current thread, using the [`thread::current`] function."] # [doc = ""] # [doc = " The [`thread::current`] function is available even for threads not spawned"] # [doc = " by the APIs of this module."] # [doc = ""] # [doc = " There is usually no need to create a `Thread` struct yourself, one"] # [doc = " should instead use a function like `spawn` to create new threads, see the"] # [doc = " docs of [`Builder`] and [`spawn`] for more details."] # [doc = ""] # [doc = " [`thread::current`]: current::current"] pub struct Thread { inner : Pin < Arc < Inner > > , }
};
}
