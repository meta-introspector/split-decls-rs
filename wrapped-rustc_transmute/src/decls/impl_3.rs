macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (feature = "rustc")] impl From < rustc_abi :: Endian > for Endian { fn from (order : rustc_abi :: Endian) -> Endian { match order { rustc_abi :: Endian :: Little => Endian :: Little , rustc_abi :: Endian :: Big => Endian :: Big , } } }
    };
}

impl_3!()