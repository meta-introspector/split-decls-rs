macro_rules! free_func {
    () => {
        pub type free_func = unsafe extern "C" fn (voidpf , voidpf) ;
    };
}

free_func!()