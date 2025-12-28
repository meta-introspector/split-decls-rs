macro_rules! de {
    () => {
        # [cfg (feature = "serde")] # [cfg (feature = "alloc")] pub mod de ;
    };
}

de!()