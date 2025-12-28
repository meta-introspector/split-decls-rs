macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < Size > PartialEq for EncodedPoint < Size > where Size : ModulusSize , { fn eq (& self , other : & Self) -> bool { self . as_bytes () == other . as_bytes () } }
    };
}

impl_11!()