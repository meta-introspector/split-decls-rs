macro_rules! deps {
    () => {
        Error!();
        Result!();
        EcPrivateKey!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a [u8] > for EcPrivateKey < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < EcPrivateKey < 'a > > { Ok (Self :: from_der (bytes) ?) } }
    };
}

impl_54!()