macro_rules! z_streamp {
    () => {
        pub type z_streamp = * mut z_stream ;
    };
}

z_streamp!()