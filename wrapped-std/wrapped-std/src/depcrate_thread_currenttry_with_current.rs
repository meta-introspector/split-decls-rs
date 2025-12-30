// Generated macro for try_with_current (function)
macro_rules! Depcrate_thread_currenttry_with_current {
() => {
// Module: crate::thread::current
// Provides: {"try_with_current"}
// Dependencies: {}
# [doc = " Gets a reference to the handle of the thread that invokes it, if the handle"] # [doc = " has been initialized."] pub (super) fn try_with_current < F , R > (f : F) -> R where F : FnOnce (Option < & Thread >) -> R , { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; f (Some (& current)) } } else { f (None) } }
};
}
