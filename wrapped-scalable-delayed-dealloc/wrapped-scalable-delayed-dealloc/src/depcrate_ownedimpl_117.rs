// Generated macro for impl_117 (impl)
macro_rules! Depcrate_ownedimpl_117 {
() => {
// Module: crate::owned
// Provides: {"impl_117"}
// Dependencies: {}
impl < T : 'static > Owned < T > { # [doc = " Creates a new instance of [`Owned`]."] # [doc = ""] # [doc = " The type of the instance must be determined at compile-time, must not contain non-static"] # [doc = " references, and must not be a non-static reference since the instance can theoretically"] # [doc = " survive the process. For instance, `struct Disallowed<'l, T>(&'l T)` is not allowed,"] # [doc = " because an instance of the type cannot outlive `'l` whereas the garbage collector does not"] # [doc = " guarantee that the instance is dropped within `'l`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sdd::Owned;"] # [doc = ""] # [doc = " let owned: Owned<usize> = Owned::new(31);"] # [doc = " ```"] # [inline] pub fn new (t : T) -> Self { Self { instance_ptr : RefCounted :: new_unique (t) , } } }
};
}
