macro_rules! zalloc_c {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " This function is safe, but must have this type signature to be used elsewhere in the library"] # [cfg (not (unix))] unsafe extern "C" fn zalloc_c (opaque : * mut c_void , items : c_uint , size : c_uint) -> * mut c_void { let _ = opaque ; let size = items as size_t * size as size_t ; if size == 0 { return core :: ptr :: null_mut () ; } extern "C" { fn malloc (size : size_t) -> * mut c_void ; } unsafe { malloc (size) } }
    };
}

zalloc_c!()