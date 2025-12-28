macro_rules! huff0 {
    () => {
        # [cfg (not (feature = "fuzz_exports"))] pub (crate) mod huff0 ;
    };
}

huff0!()