// Generated macro for impl_191 (impl)
macro_rules! Depcrate_lib_generatedimpl_191 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_191"}
// Dependencies: {}
impl Eventtype { pub const fn raw (& self) -> u8 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "CLOCK" , 1 => "FD_READ" , 2 => "FD_WRITE" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => { "The time value of clock `subscription_clock::id` has
reached timestamp `subscription_clock::timeout`." } 1 => { "File descriptor `subscription_fd_readwrite::file_descriptor` has data
available for reading. This event always triggers for regular files." } 2 => { "File descriptor `subscription_fd_readwrite::file_descriptor` has capacity
available for writing. This event always triggers for regular files." } _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
