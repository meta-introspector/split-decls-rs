macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl digest :: zeroize :: ZeroizeOnDrop for Sha256VarCore { }
    };
}

impl_11!();