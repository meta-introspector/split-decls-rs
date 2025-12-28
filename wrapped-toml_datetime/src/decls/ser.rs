macro_rules! ser {
    () => {
        # [cfg (feature = "serde")] # [cfg (feature = "alloc")] pub mod ser ;
    };
}

ser!();