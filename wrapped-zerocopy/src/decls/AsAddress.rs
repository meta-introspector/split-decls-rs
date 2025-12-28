macro_rules! AsAddress {
    () => {
        pub (crate) trait AsAddress { fn addr (self) -> usize ; }
    };
}

AsAddress!();