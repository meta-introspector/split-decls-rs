// Generated macro for RefCounted (struct)
macro_rules! Depcrate_ref_countedRefCounted {
() => {
// Module: crate::ref_counted
// Provides: {"RefCounted"}
// Dependencies: {}
# [doc = " [`RefCounted`] stores an instance of type `T`, and a union of a link to the next"] # [doc = " [`Collectible`] or the reference counter."] pub (super) struct RefCounted < T > { instance : T , next_or_refcnt : Link , }
};
}
