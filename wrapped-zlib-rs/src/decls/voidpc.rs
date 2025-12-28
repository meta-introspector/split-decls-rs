macro_rules! voidpc {
    () => {
        pub type voidpc = * const c_void ;
    };
}

voidpc!()