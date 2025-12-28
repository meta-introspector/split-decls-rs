macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < Size : ModulusSize > Ord for EncodedPoint < Size > where Size : ModulusSize , { fn cmp (& self , other : & Self) -> Ordering { self . as_bytes () . cmp (other . as_bytes ()) } }
    };
}

impl_14!()