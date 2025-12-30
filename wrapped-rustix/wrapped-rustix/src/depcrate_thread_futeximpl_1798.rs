// Generated macro for impl_1798 (impl)
macro_rules! Depcrate_thread_futeximpl_1798 {
() => {
// Module: crate::thread::futex
// Provides: {"impl_1798"}
// Dependencies: {}
impl Wait { # [doc = " Construct a zero-initialized `Wait`."] # [inline] pub const fn new () -> Self { Self { val : 0 , uaddr : WaitPtr :: new (ptr :: null_mut ()) , flags : WaitFlags :: empty () , __reserved : 0 , } } }
};
}
