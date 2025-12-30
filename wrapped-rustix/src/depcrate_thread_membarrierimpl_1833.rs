// Generated macro for impl_1833 (impl)
macro_rules! Depcrate_thread_membarrierimpl_1833 {
() => {
// Module: crate::thread::membarrier
// Provides: {"impl_1833"}
// Dependencies: {}
# [cfg (linux_kernel)] impl MembarrierQuery { # [doc = " Test whether this query result contains the given command."] # [inline] pub fn contains_command (self , cmd : MembarrierCommand) -> bool { self . contains (Self :: from_bits_retain (cmd as _)) } }
};
}
