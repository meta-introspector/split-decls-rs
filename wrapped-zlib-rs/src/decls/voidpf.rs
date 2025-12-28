macro_rules! voidpf {
    () => {
        pub type voidpf = * mut c_void ;
    };
}

voidpf!();