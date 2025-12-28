macro_rules! deps {
    () => {
        Result!();
        EcParameters!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl EncodeValue for EcParameters { fn value_len (& self) -> der :: Result < Length > { match self { Self :: NamedCurve (oid) => oid . value_len () , } } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { match self { Self :: NamedCurve (oid) => oid . encode_value (writer) , } } }
    };
}

impl_40!();