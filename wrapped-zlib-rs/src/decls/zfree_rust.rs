macro_rules! zfree_rust {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " - `ptr` must be allocated with the rust `alloc::System` allocator"] # [doc = " - `opaque` is a `&usize` that represents the size of the allocation"] # [cfg (feature = "rust-allocator")] unsafe extern "C" fn zfree_rust (opaque : * mut c_void , ptr : * mut c_void) { if ptr . is_null () { return ; } debug_assert ! (! opaque . is_null ()) ; if opaque . is_null () { return ; } let size = unsafe { * (opaque as * mut usize) } ; if size == 0 { return ; } let layout = Layout :: from_size_align (size , ALIGN . into ()) ; let layout = layout . unwrap () ; unsafe { std :: alloc :: System . dealloc (ptr . cast () , layout) } ; }
    };
}

zfree_rust!();