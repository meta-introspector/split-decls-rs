macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < Size > AsRef < [u8] > for EncodedPoint < Size > where Size : ModulusSize , { # [inline] fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_6!()