macro_rules! deps {
    () => {
        Result!();
        EcParameters!();
        Error!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a > DecodeValue < 'a > for EcParameters { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (decoder : & mut R , header : Header) -> der :: Result < Self > { ObjectIdentifier :: decode_value (decoder , header) . map (Self :: NamedCurve) } }
    };
}

impl_39!()