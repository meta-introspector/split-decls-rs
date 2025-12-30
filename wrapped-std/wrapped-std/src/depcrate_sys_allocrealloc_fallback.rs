// Generated macro for realloc_fallback (function)
macro_rules! Depcrate_sys_allocrealloc_fallback {
() => {
// Module: crate::sys::alloc
// Provides: {"realloc_fallback"}
// Dependencies: {}
# [allow (dead_code)] unsafe fn realloc_fallback (alloc : & System , ptr : * mut u8 , old_layout : Layout , new_size : usize ,) -> * mut u8 { unsafe { let new_layout = Layout :: from_size_align_unchecked (new_size , old_layout . align ()) ; let new_ptr = GlobalAlloc :: alloc (alloc , new_layout) ; if ! new_ptr . is_null () { let size = usize :: min (old_layout . size () , new_size) ; ptr :: copy_nonoverlapping (ptr , new_ptr , size) ; GlobalAlloc :: dealloc (alloc , ptr , old_layout) ; } new_ptr } }
};
}
