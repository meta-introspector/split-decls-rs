// Generated macro for zalloc_rust_calloc (function)
macro_rules! Depcrate_allocatezalloc_rust_calloc {
() => {
// Module: crate::allocate
// Provides: {"zalloc_rust_calloc"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " This function is safe to call."] # [cfg (feature = "rust-allocator")] unsafe extern "C" fn zalloc_rust_calloc (_opaque : * mut c_void , count : c_uint , size : c_uint ,) -> * mut c_void { let size = count as usize * size as usize ; if size == 0 { return core :: ptr :: null_mut () ; } let layout = Layout :: from_size_align (size , ALIGN . into ()) . unwrap () ; let ptr = unsafe { std :: alloc :: System . alloc_zeroed (layout) } ; ptr as * mut c_void }
};
}
