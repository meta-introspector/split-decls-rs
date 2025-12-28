macro_rules! repr_to_ptr_mut {
    () => {
        fn repr_to_ptr_mut (repr : NonNull < u8 >) -> * mut u8 { repr_to_ptr (repr) as * mut u8 }
    };
}

repr_to_ptr_mut!()