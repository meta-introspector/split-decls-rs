// Generated macro for Shared (enum)
macro_rules! Depcrate_pointer_invariantShared {
() => {
// Module: crate::pointer::invariant
// Provides: {"Shared"}
// Dependencies: {}
# [doc = " The `Ptr<'a, T>` adheres to the aliasing rules of a `&'a T`."] # [doc = ""] # [doc = " The referent of a shared-aliased `Ptr` may be concurrently referenced by any"] # [doc = " number of shared-aliased `Ptr` or `&T` references, or by any number of"] # [doc = " `Ptr<U>` or `&U` references as permitted by `T`'s library safety invariants,"] # [doc = " and may not be concurrently referenced by any exclusively-aliased `Ptr`s or"] # [doc = " `&mut` references. The referent must not be mutated, except via"] # [doc = " [`UnsafeCell`]s, and only when permitted by `T`'s library safety invariants."] # [doc = ""] # [doc = " [`UnsafeCell`]: core::cell::UnsafeCell"] pub enum Shared { }
};
}
