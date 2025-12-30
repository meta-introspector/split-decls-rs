// Generated macro for Stack (struct)
macro_rules! Depcrate_stackStack {
() => {
// Module: crate::stack
// Provides: {"Stack"}
// Dependencies: {}
# [doc = " [`Stack`] is a lock-free concurrent last-in-first-out container."] pub struct Stack < T > { # [doc = " `newest` points to the newest entry in the [`Stack`]."] newest : AtomicShared < LinkedEntry < T > > , }
};
}
