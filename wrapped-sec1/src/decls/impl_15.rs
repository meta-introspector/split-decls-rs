macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
        Error!();
        Result!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < Size : ModulusSize > TryFrom < & [u8] > for EncodedPoint < Size > where Size : ModulusSize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_bytes (bytes) } }
    };
}

impl_15!();