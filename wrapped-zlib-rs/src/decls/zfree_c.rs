macro_rules! zfree_c {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " The `ptr` must be allocated with the allocator that is used internally by `zcfree`"] unsafe extern "C" fn zfree_c (opaque : * mut c_void , ptr : * mut c_void) { let _ = opaque ; unsafe extern "C" { fn free (p : * mut c_void) ; } unsafe { free (ptr) } }
    };
}

zfree_c!();