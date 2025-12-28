macro_rules! deps {
    () => {
        Result!();
        EcPrivateKey!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl EncodeValue for EcPrivateKey < '_ > { fn value_len (& self) -> der :: Result < Length > { VERSION . encoded_len () ? + OctetStringRef :: new (self . private_key) ? . encoded_len () ? + self . context_specific_parameters () . encoded_len () ? + self . context_specific_public_key () ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { VERSION . encode (writer) ? ; OctetStringRef :: new (self . private_key) ? . encode (writer) ? ; self . context_specific_parameters () . encode (writer) ? ; self . context_specific_public_key () ? . encode (writer) ? ; Ok (()) } }
    };
}

impl_52!()