macro_rules! gz_headerp {
    () => {
        pub type gz_headerp = * mut gz_header ;
    };
}

gz_headerp!();