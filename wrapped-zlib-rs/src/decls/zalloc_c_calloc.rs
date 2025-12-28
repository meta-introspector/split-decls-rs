macro_rules! zalloc_c_calloc {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " This function is safe, but must have this type signature to be used elsewhere in the library"] unsafe extern "C" fn zalloc_c_calloc (opaque : * mut c_void , items : c_uint , size : c_uint ,) -> * mut c_void { let _ = opaque ; unsafe extern "C" { fn calloc (nitems : size_t , size : size_t) -> * mut c_void ; } if items as size_t * size as size_t == 0 { return core :: ptr :: null_mut () ; } unsafe { calloc (items as size_t , size as size_t) } }
    };
}

zalloc_c_calloc!();