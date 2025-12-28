macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < Size > Zeroize for EncodedPoint < Size > where Size : ModulusSize , { fn zeroize (& mut self) { self . bytes . zeroize () ; * self = Self :: identity () ; } }
    };
}

impl_16!()