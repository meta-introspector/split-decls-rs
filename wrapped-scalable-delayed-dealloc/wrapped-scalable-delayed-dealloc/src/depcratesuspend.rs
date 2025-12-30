// Generated macro for suspend (function)
macro_rules! Depcratesuspend {
() => {
// Module: crate
// Provides: {"suspend"}
// Dependencies: {}
# [doc = " Suspends the garbage collector of the current thread."] # [doc = ""] # [doc = " It returns `false` if there is an active [`Guard`] in the thread. Otherwise, it passes all its"] # [doc = " retired instances to a free flowing garbage container that can be cleaned up by other threads."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sdd::{suspend, Guard};"] # [doc = ""] # [doc = " assert!(suspend());"] # [doc = ""] # [doc = " {"] # [doc = "     let guard = Guard::new();"] # [doc = "     assert!(!suspend());"] # [doc = " }"] # [doc = ""] # [doc = " assert!(suspend());"] # [doc = " ```"] # [inline] # [must_use] pub fn suspend () -> bool { collector :: Collector :: pass_garbage () }
};
}
