// Generated macro for impl_1793 (impl)
macro_rules! Depcrate_thread_futeximpl_1793 {
() => {
// Module: crate::thread::futex
// Provides: {"impl_1793"}
// Dependencies: {}
impl WaitPtr { # [doc = " Construct a new `WaitPtr` holding the given raw pointer value."] # [inline] pub const fn new (ptr : * mut c_void) -> Self { Self { ptr , # [cfg (target_pointer_width = "16")] __pad16 : 0 , # [cfg (any (target_pointer_width = "16" , target_pointer_width = "32"))] __pad32 : 0 , } } }
};
}
