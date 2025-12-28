macro_rules! deps {
    () => {
        BarrierState!();
        Mutex!();
        Condvar!();
    };
}

macro_rules! Barrier {
    () => {
        deps!();
        # [doc = " A barrier enables multiple threads to synchronize the beginning"] # [doc = " of some computation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(not(target_family = \"wasm\"))]"] # [doc = " # {"] # [doc = " use std::sync::{Arc, Barrier};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mut handles = Vec::with_capacity(10);"] # [doc = " let barrier = Arc::new(Barrier::new(10));"] # [doc = " for _ in 0..10 {"] # [doc = "     let c = Arc::clone(&barrier);"] # [doc = "     // The same messages will be printed together."] # [doc = "     // You will NOT see any interleaving."] # [doc = "     handles.push(thread::spawn(move|| {"] # [doc = "         println!(\"before wait\");"] # [doc = "         c.wait();"] # [doc = "         println!(\"after wait\");"] # [doc = "     }));"] # [doc = " }"] # [doc = " // Wait for other threads to finish."] # [doc = " for handle in handles {"] # [doc = "     handle.join().unwrap();"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] pub (crate) struct Barrier { lock : Mutex < BarrierState > , cvar : Condvar , num_threads : usize , }
    };
}

Barrier!();