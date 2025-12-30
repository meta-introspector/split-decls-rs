// Generated macro for ArcBorrow (struct)
macro_rules! Depcrate_arc_borrowArcBorrow {
() => {
// Module: crate::arc_borrow
// Provides: {"ArcBorrow"}
// Dependencies: {}
# [doc = " A \"borrowed `Arc`\". This is a pointer to"] # [doc = " a T that is known to have been allocated within an"] # [doc = " `Arc`."] # [doc = ""] # [doc = " This is equivalent in guarantees to `&Arc<T>`, however it is"] # [doc = " a bit more flexible. To obtain an `&Arc<T>` you must have"] # [doc = " an `Arc<T>` instance somewhere pinned down until we're done with it."] # [doc = " It's also a direct pointer to `T`, so using this involves less pointer-chasing"] # [doc = ""] # [doc = " However, C++ code may hand us refcounted things as pointers to T directly,"] # [doc = " so we have to conjure up a temporary `Arc` on the stack each time. The"] # [doc = " same happens for when the object is managed by a `OffsetArc`."] # [doc = ""] # [doc = " `ArcBorrow` lets us deal with borrows of known-refcounted objects"] # [doc = " without needing to worry about where the `Arc<T>` is."] # [derive (Debug , Eq , PartialEq)] # [repr (transparent)] pub struct ArcBorrow < 'a , T : ? Sized + 'a > (pub (crate) NonNull < T > , pub (crate) PhantomData < & 'a T >) ;
};
}
