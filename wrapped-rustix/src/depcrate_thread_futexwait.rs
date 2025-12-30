// Generated macro for Wait (struct)
macro_rules! Depcrate_thread_futexWait {
() => {
// Module: crate::thread::futex
// Provides: {"Wait"}
// Dependencies: {}
# [doc = " For use with [`waitv`]."] # [repr (C)] # [derive (Debug , Copy , Clone)] # [non_exhaustive] pub struct Wait { # [doc = " The expected value."] pub val : u64 , # [doc = " The address to wait for."] pub uaddr : WaitPtr , # [doc = " The type and size of futex to perform."] pub flags : WaitFlags , # [doc = " Reserved for future use."] pub (crate) __reserved : u32 , }
};
}
