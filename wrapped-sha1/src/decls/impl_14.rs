macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl ZeroizeOnDrop for Sha1Core { }
    };
}

impl_14!()