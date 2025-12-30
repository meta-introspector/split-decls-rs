// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl SharedGiver { # [doc = " Check if the `Taker` has called `want()` without parking a task."] # [doc = ""] # [doc = " This is safe to call outside of a futures task context, but other"] # [doc = " means of being notified is left to the user."] # [inline] pub fn is_wanting (& self) -> bool { self . inner . state . load (SeqCst) == State :: Want . into () } # [doc = " Check if the `Taker` has canceled interest without parking a task."] # [inline] pub fn is_canceled (& self) -> bool { self . inner . state . load (SeqCst) == State :: Closed . into () } }
};
}
