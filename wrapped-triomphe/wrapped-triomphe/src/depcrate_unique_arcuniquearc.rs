// Generated macro for UniqueArc (struct)
macro_rules! Depcrate_unique_arcUniqueArc {
() => {
// Module: crate::unique_arc
// Provides: {"UniqueArc"}
// Dependencies: {}
# [doc = " An `Arc` that is known to be uniquely owned"] # [doc = ""] # [doc = " When `Arc`s are constructed, they are known to be"] # [doc = " uniquely owned. In such a case it is safe to mutate"] # [doc = " the contents of the `Arc`. Normally, one would just handle"] # [doc = " this by mutating the data on the stack before allocating the"] # [doc = " `Arc`, however it's possible the data is large or unsized"] # [doc = " and you need to heap-allocate it earlier in such a way"] # [doc = " that it can be freely converted into a regular `Arc` once you're"] # [doc = " done."] # [doc = ""] # [doc = " `UniqueArc` exists for this purpose, when constructed it performs"] # [doc = " the same allocations necessary for an `Arc`, however it allows mutable access."] # [doc = " Once the mutation is finished, you can call `.shareable()` and get a regular `Arc`"] # [doc = " out of it."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use triomphe::UniqueArc;"] # [doc = " let data = [1, 2, 3, 4, 5];"] # [doc = " let mut x = UniqueArc::new(data);"] # [doc = " x[4] = 7; // mutate!"] # [doc = " let y = x.shareable(); // y is an Arc<T>"] # [doc = " ```"] # [repr (transparent)] pub struct UniqueArc < T : ? Sized > (Arc < T >) ;
};
}
