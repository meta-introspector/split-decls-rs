// Generated macro for ErasedDestructor (trait)
macro_rules! Depcrate_erasedErasedDestructor {
() => {
// Module: crate::erased
// Provides: {"ErasedDestructor"}
// Dependencies: {}
# [doc = " Dummy trait that lets us `dyn Drop`"] # [doc = ""] # [doc = " `dyn Drop` isn't legal (and doesn't make sense since `Drop` is not"] # [doc = " implement on all destructible types). However, all trait objects come with"] # [doc = " a destructor, so we can just use an empty trait to get a destructor object."] pub trait ErasedDestructor : 'static { }
};
}
