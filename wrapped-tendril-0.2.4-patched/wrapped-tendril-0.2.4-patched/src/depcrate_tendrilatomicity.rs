// Generated macro for Atomicity (trait)
macro_rules! Depcrate_tendrilAtomicity {
() => {
// Module: crate::tendril
// Provides: {"Atomicity"}
// Dependencies: {}
# [doc = " The multithreadedness of a tendril."] # [doc = ""] # [doc = " Exactly two types implement this trait:"] # [doc = ""] # [doc = " - `Atomic`: use this in your tendril and you will have a `Send + Sync` tendril which works"] # [doc = "   across threads; this is akin to `Arc`."] # [doc = ""] # [doc = " - `NonAtomic`: use this in your tendril and you will have a tendril which is neither"] # [doc = "   `Send` nor `Sync` but should be a tad faster; this is akin to `Rc`."] # [doc = ""] # [doc = " The layout of this trait is also mandated to be that of a `usize`,"] # [doc = " for it is used for reference counting."] pub unsafe trait Atomicity : 'static { # [doc (hidden)] fn new () -> Self ; # [doc (hidden)] fn increment (& self) -> usize ; # [doc (hidden)] fn decrement (& self) -> usize ; # [doc (hidden)] fn fence_acquire () ; }
};
}
