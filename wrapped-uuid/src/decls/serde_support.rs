macro_rules! serde_support {
    () => {
        # [cfg (feature = "serde")] pub (crate) mod serde_support ;
    };
}

serde_support!()