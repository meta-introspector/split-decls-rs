// Generated macro for Barrier (struct)
macro_rules! Depcrate_sync_barrierBarrier {
() => {
// Module: crate::sync::barrier
// Provides: {"Barrier"}
// Dependencies: {}
# [doc = " A barrier enables multiple threads to synchronize the beginning"] # [doc = " of some computation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Barrier;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let n = 10;"] # [doc = " let barrier = Barrier::new(n);"] # [doc = " thread::scope(|s| {"] # [doc = "     for _ in 0..n {"] # [doc = "         // The same messages will be printed together."] # [doc = "         // You will NOT see any interleaving."] # [doc = "         s.spawn(|| {"] # [doc = "             println!(\"before wait\");"] # [doc = "             barrier.wait();"] # [doc = "             println!(\"after wait\");"] # [doc = "         });"] # [doc = "     }"] # [doc = " });"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Barrier { lock : Mutex < BarrierState > , cvar : Condvar , num_threads : usize , }
};
}
