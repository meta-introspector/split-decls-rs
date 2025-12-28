macro_rules! errors {
    () => {
        # [cfg (feature = "rustc")] pub mod errors ;
    };
}

errors!()