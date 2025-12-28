macro_rules! impl_391 {
    () => {
        impl From < * mut c_void > for io_uring_user_data { # [inline] fn from (ptr : * mut c_void) -> Self { Self :: from_ptr (ptr) } }
    };
}

impl_391!();