macro_rules! fallback_cgu_name {
    () => {
        fn fallback_cgu_name (name_builder : & mut CodegenUnitNameBuilder < '_ >) -> Symbol { name_builder . build_cgu_name (LOCAL_CRATE , & ["fallback"] , Some ("cgu")) }
    };
}

fallback_cgu_name!()