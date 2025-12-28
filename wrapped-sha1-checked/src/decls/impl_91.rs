macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl ZeroizeOnDrop for Sha1 { }
    };
}

impl_91!()