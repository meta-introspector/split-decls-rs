macro_rules! ser {
    () => {
        # [cfg (feature = "serde")] pub mod ser ;
    };
}

ser!()