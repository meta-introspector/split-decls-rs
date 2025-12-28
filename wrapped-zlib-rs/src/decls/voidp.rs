macro_rules! voidp {
    () => {
        pub type voidp = * mut c_void ;
    };
}

voidp!()