macro_rules! deps {
    () => {
        DeserializerError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl DeserializerError { pub fn to_error < E : de :: Error > (& self) -> E { match * self { DeserializerError :: Custom (ref msg) => E :: custom (msg . clone ()) , DeserializerError :: InvalidType (ref unexp , ref exp) => { E :: invalid_type (unexp . to_unexpected () , & & * * exp) } DeserializerError :: InvalidValue (ref unexp , ref exp) => { E :: invalid_value (unexp . to_unexpected () , & & * * exp) } DeserializerError :: InvalidLength (len , ref exp) => E :: invalid_length (len , & & * * exp) , DeserializerError :: UnknownVariant (ref field , exp) => E :: unknown_variant (field , exp) , DeserializerError :: UnknownField (ref field , exp) => E :: unknown_field (field , exp) , DeserializerError :: MissingField (field) => E :: missing_field (field) , DeserializerError :: DuplicateField (field) => E :: missing_field (field) , } } pub fn into_error < E : de :: Error > (self) -> E { self . to_error () } }
    };
}

impl_5!()