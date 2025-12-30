// Generated macro for PoisonError (struct)
macro_rules! Depcrate_sync_poisonPoisonError {
() => {
// Module: crate::sync::poison
// Provides: {"PoisonError"}
// Dependencies: {}
# [doc = " A type of error which can be returned whenever a lock is acquired."] # [doc = ""] # [doc = " Both [`Mutex`]es and [`RwLock`]s are poisoned whenever a thread fails while the lock"] # [doc = " is held. The precise semantics for when a lock is poisoned is documented on"] # [doc = " each lock. For a lock in the poisoned state, unless the state is cleared manually,"] # [doc = " all future acquisitions will return this error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(1));"] # [doc = ""] # [doc = " // poison the mutex"] # [doc = " let c_mutex = Arc::clone(&mutex);"] # [doc = " let _ = thread::spawn(move || {"] # [doc = "     let mut data = c_mutex.lock().unwrap();"] # [doc = "     *data = 2;"] # [doc = "     panic!();"] # [doc = " }).join();"] # [doc = ""] # [doc = " match mutex.lock() {"] # [doc = "     Ok(_) => unreachable!(),"] # [doc = "     Err(p_err) => {"] # [doc = "         let data = p_err.get_ref();"] # [doc = "         println!(\"recovered: {data}\");"] # [doc = "     }"] # [doc = " };"] # [doc = " ```"] # [doc = " [`Mutex`]: crate::sync::Mutex"] # [doc = " [`RwLock`]: crate::sync::RwLock"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct PoisonError < T > { data : T , # [cfg (not (panic = "unwind"))] _never : ! , }
};
}
