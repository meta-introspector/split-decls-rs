// Generated macro for impl_29 (impl)
macro_rules! Depcrate_atomic_sharedimpl_29 {
() => {
// Module: crate::atomic_shared
// Provides: {"impl_29"}
// Dependencies: {}
impl < T : 'static > AtomicShared < T > { # [doc = " Creates a new [`AtomicShared`] from an instance of `T`."] # [doc = ""] # [doc = " The type of the instance must be determined at compile-time, must not contain non-static"] # [doc = " references, and must not be a non-static reference since the instance can theoretically"] # [doc = " live as long as the process. For instance, `struct Disallowed<'l, T>(&'l T)` is not"] # [doc = " allowed, because an instance of the type cannot outlive `'l` whereas the garbage collector"] # [doc = " does not guarantee that the instance is dropped within `'l`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sdd::AtomicShared;"] # [doc = ""] # [doc = " let atomic_shared: AtomicShared<usize> = AtomicShared::new(10);"] # [doc = " ```"] # [inline] pub fn new (t : T) -> Self { Self { instance_ptr : AtomicPtr :: new (RefCounted :: new_shared (t) . as_ptr ()) , } } }
};
}
