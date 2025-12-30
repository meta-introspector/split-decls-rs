// Generated macro for impl_12 (impl)
macro_rules! Depcrate_lib_generatedimpl_12 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_12"}
// Dependencies: {}
impl Clockid { pub const fn raw (& self) -> u32 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "REALTIME" , 1 => "MONOTONIC" , 2 => "PROCESS_CPUTIME_ID" , 3 => "THREAD_CPUTIME_ID" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => { "The clock measuring real time. Time value zero corresponds with
1970-01-01T00:00:00Z." } 1 => { "The store-wide monotonic clock, which is defined as a clock measuring
real time, whose value cannot be adjusted and which cannot have negative
clock jumps. The epoch of this clock is undefined. The absolute time
value of this clock therefore has no meaning." } 2 => "The CPU-time clock associated with the current process." , 3 => "The CPU-time clock associated with the current thread." , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
