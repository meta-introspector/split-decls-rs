// Generated macro for OffsetArc (struct)
macro_rules! Depcrate_offset_arcOffsetArc {
() => {
// Module: crate::offset_arc
// Provides: {"OffsetArc"}
// Dependencies: {}
# [doc = " An `Arc`, except it holds a pointer to the T instead of to the"] # [doc = " entire ArcInner."] # [doc = ""] # [doc = " An `OffsetArc<T>` has the same layout and ABI as a non-null"] # [doc = " `const T*` in C, and may be used in FFI function signatures."] # [doc = ""] # [doc = " ```text"] # [doc = "  Arc<T>    OffsetArc<T>"] # [doc = "   |          |"] # [doc = "   v          v"] # [doc = "  ---------------------"] # [doc = " | RefCount | T (data) | [ArcInner<T>]"] # [doc = "  ---------------------"] # [doc = " ```"] # [doc = ""] # [doc = " This means that this is a direct pointer to"] # [doc = " its contained data (and can be read from by both C++ and Rust),"] # [doc = " but we can also convert it to a \"regular\" `Arc<T>` by removing the offset."] # [doc = ""] # [doc = " This is very useful if you have an Arc-containing struct shared between Rust and C++,"] # [doc = " and wish for C++ to be able to read the data behind the `Arc` without incurring"] # [doc = " an FFI call overhead."] # [derive (Eq)] # [repr (transparent)] pub struct OffsetArc < T > { pub (crate) ptr : ptr :: NonNull < T > , pub (crate) phantom : PhantomData < T > , }
};
}
