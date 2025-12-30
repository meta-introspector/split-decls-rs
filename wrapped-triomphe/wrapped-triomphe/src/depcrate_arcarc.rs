// Generated macro for Arc (struct)
macro_rules! Depcrate_arcArc {
() => {
// Module: crate::arc
// Provides: {"Arc"}
// Dependencies: {}
# [doc = " An atomically reference counted shared pointer"] # [doc = ""] # [doc = " See the documentation for [`Arc`] in the standard library. Unlike the"] # [doc = " standard library `Arc`, this `Arc` does not support weak reference counting."] # [doc = ""] # [doc = " [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html"] # [repr (transparent)] pub struct Arc < T : ? Sized > { pub (crate) p : ptr :: NonNull < ArcInner < T > > , pub (crate) phantom : PhantomData < T > , }
};
}
