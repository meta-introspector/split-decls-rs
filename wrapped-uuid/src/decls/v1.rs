macro_rules! v1 {
    () => {
        # [cfg (feature = "v1")] # [doc (hidden)] pub mod v1 ;
    };
}

v1!();