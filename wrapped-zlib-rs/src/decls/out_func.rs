macro_rules! out_func {
    () => {
        pub type out_func = unsafe extern "C" fn (* mut c_void , * mut c_uchar , c_uint) -> c_int ;
    };
}

out_func!();