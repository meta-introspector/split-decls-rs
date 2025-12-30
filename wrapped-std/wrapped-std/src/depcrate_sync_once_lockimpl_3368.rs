// Generated macro for impl_3368 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3368 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3368"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : PartialEq > PartialEq for OnceLock < T > { # [doc = " Equality for two `OnceLock`s."] # [doc = ""] # [doc = " Two `OnceLock`s are equal if they either both contain values and their"] # [doc = " values are equal, or if neither contains a value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let five = OnceLock::new();"] # [doc = " five.set(5).unwrap();"] # [doc = ""] # [doc = " let also_five = OnceLock::new();"] # [doc = " also_five.set(5).unwrap();"] # [doc = ""] # [doc = " assert!(five == also_five);"] # [doc = ""] # [doc = " assert!(OnceLock::<u32>::new() == OnceLock::<u32>::new());"] # [doc = " ```"] # [inline] fn eq (& self , other : & OnceLock < T >) -> bool { self . get () == other . get () } }
};
}
