macro_rules! impl_379 {
    () => {
        impl From < * mut c_void > for io_uring_ptr { # [inline] fn from (ptr : * mut c_void) -> Self { Self :: new (ptr) } }
    };
}

impl_379!();