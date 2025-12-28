macro_rules! deps {
    () => {
        InvertedUnsigned!();
        InvertedUInt!();
        Bit!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned , B : Bit > InvertedUnsigned for InvertedUInt < IU , B > { # [inline] fn to_u64 () -> u64 { u64 :: from (B :: to_u8 ()) | IU :: to_u64 () << 1 } }
    };
}

impl_238!()