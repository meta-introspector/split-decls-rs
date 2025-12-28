macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < Size : ModulusSize > PartialOrd for EncodedPoint < Size > where Size : ModulusSize , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_13!();