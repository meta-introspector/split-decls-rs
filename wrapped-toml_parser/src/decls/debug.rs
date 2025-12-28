macro_rules! debug {
    () => {
        # [cfg (feature = "debug")] pub (crate) mod debug ;
    };
}

debug!()