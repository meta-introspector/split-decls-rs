macro_rules! deps {
    () => {
        Value!();
        ValueDeserializer!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < E > ValueDeserializer < E > { pub fn new (value : Value) -> Self { ValueDeserializer { value : value , error : Default :: default () , } } pub fn into_value (self) -> Value { self . value } }
    };
}

impl_14!()