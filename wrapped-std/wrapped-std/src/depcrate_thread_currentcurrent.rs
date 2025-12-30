// Generated macro for current (function)
macro_rules! Depcrate_thread_currentcurrent {
() => {
// Module: crate::thread::current
// Provides: {"current"}
// Dependencies: {}
# [doc = " Gets a handle to the thread that invokes it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Getting a handle to the current thread with `thread::current()`:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let handler = thread::Builder::new()"] # [doc = "     .name(\"named thread\".into())"] # [doc = "     .spawn(|| {"] # [doc = "         let handle = thread::current();"] # [doc = "         assert_eq!(handle.name(), Some(\"named thread\"));"] # [doc = "     })"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " handler.join().unwrap();"] # [doc = " ```"] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn current () -> Thread { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; (* current) . clone () } } else { init_current (current) } }
};
}
