macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl digest :: zeroize :: ZeroizeOnDrop for Sha512VarCore { }
    };
}

impl_23!()