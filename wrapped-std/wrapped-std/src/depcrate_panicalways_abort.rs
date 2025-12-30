// Generated macro for always_abort (function)
macro_rules! Depcrate_panicalways_abort {
() => {
// Module: crate::panic
// Provides: {"always_abort"}
// Dependencies: {}
# [doc = " Makes all future panics abort directly without running the panic hook or unwinding."] # [doc = ""] # [doc = " There is no way to undo this; the effect lasts until the process exits or"] # [doc = " execs (or the equivalent)."] # [doc = ""] # [doc = " # Use after fork"] # [doc = ""] # [doc = " This function is particularly useful for calling after `libc::fork`.  After `fork`, in a"] # [doc = " multithreaded program it is (on many platforms) not safe to call the allocator.  It is also"] # [doc = " generally highly undesirable for an unwind to unwind past the `fork`, because that results in"] # [doc = " the unwind propagating to code that was only ever expecting to run in the parent."] # [doc = ""] # [doc = " `panic::always_abort()` helps avoid both of these.  It directly avoids any further unwinding,"] # [doc = " and if there is a panic, the abort will occur without allocating provided that the arguments to"] # [doc = " panic can be formatted without allocating."] # [doc = ""] # [doc = " Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(panic_always_abort)]"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::always_abort();"] # [doc = ""] # [doc = " let _ = panic::catch_unwind(|| {"] # [doc = "     panic!(\"inside the catch\");"] # [doc = " });"] # [doc = ""] # [doc = " // We will have aborted already, due to the panic."] # [doc = " unreachable!();"] # [doc = " ```"] # [unstable (feature = "panic_always_abort" , issue = "84438")] pub fn always_abort () { crate :: panicking :: panic_count :: set_always_abort () ; }
};
}
