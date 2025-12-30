// Generated macro for __default_lib_allocator (module)
macro_rules! Depcrate_alloc__default_lib_allocator {
() => {
// Module: crate::alloc
// Provides: {"__default_lib_allocator"}
// Dependencies: {}
# [cfg (not (test))] # [doc (hidden)] # [allow (unused_attributes)] # [unstable (feature = "alloc_internals" , issue = "none")] pub mod __default_lib_allocator { use super :: { GlobalAlloc , Layout , System } ; # [rustc_std_internal_symbol] pub unsafe extern "C" fn __rdl_alloc (size : usize , align : usize) -> * mut u8 { unsafe { let layout = Layout :: from_size_align_unchecked (size , align) ; System . alloc (layout) } } # [rustc_std_internal_symbol] pub unsafe extern "C" fn __rdl_dealloc (ptr : * mut u8 , size : usize , align : usize) { unsafe { System . dealloc (ptr , Layout :: from_size_align_unchecked (size , align)) } } # [rustc_std_internal_symbol] pub unsafe extern "C" fn __rdl_realloc (ptr : * mut u8 , old_size : usize , align : usize , new_size : usize ,) -> * mut u8 { unsafe { let old_layout = Layout :: from_size_align_unchecked (old_size , align) ; System . realloc (ptr , old_layout , new_size) } } # [rustc_std_internal_symbol] pub unsafe extern "C" fn __rdl_alloc_zeroed (size : usize , align : usize) -> * mut u8 { unsafe { let layout = Layout :: from_size_align_unchecked (size , align) ; System . alloc_zeroed (layout) } } }
};
}
