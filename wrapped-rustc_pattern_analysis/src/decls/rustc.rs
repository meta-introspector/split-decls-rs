macro_rules! rustc {
    () => {
        # [cfg (feature = "rustc")] pub mod rustc ;
    };
}

rustc!();