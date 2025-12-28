macro_rules! in_func {
    () => {
        pub type in_func = unsafe extern "C" fn (* mut c_void , * mut * const c_uchar) -> c_uint ;
    };
}

in_func!();