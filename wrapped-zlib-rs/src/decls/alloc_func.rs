macro_rules! alloc_func {
    () => {
        pub type alloc_func = unsafe extern "C" fn (voidpf , uInt , uInt) -> voidpf ;
    };
}

alloc_func!();