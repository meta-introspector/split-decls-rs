// Generated macro for ThreadId (struct)
macro_rules! Depcrate_threadThreadId {
() => {
// Module: crate::thread
// Provides: {"ThreadId"}
// Dependencies: {}
# [doc = " A unique identifier for a running thread."] # [doc = ""] # [doc = " A `ThreadId` is an opaque object that uniquely identifies each thread"] # [doc = " created during the lifetime of a process. `ThreadId`s are guaranteed not to"] # [doc = " be reused, even when a thread terminates. `ThreadId`s are under the control"] # [doc = " of Rust's standard library and there may not be any relationship between"] # [doc = " `ThreadId` and the underlying platform's notion of a thread identifier --"] # [doc = " the two concepts cannot, therefore, be used interchangeably. A `ThreadId`"] # [doc = " can be retrieved from the [`id`] method on a [`Thread`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let other_thread = thread::spawn(|| {"] # [doc = "     thread::current().id()"] # [doc = " });"] # [doc = ""] # [doc = " let other_thread_id = other_thread.join().unwrap();"] # [doc = " assert!(thread::current().id() != other_thread_id);"] # [doc = " ```"] # [doc = ""] # [doc = " [`id`]: Thread::id"] # [stable (feature = "thread_id" , since = "1.19.0")] # [derive (Eq , PartialEq , Clone , Copy , Hash , Debug)] pub struct ThreadId (NonZero < u64 >) ;
};
}
