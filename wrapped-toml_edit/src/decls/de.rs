macro_rules! de {
    () => {
        # [cfg (feature = "serde")] pub mod de ;
    };
}

de!()