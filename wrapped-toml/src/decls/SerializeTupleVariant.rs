macro_rules! deps {
    () => {
        SerializeValueArray!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        pub struct SerializeTupleVariant < 'd > { inner : SerializeValueArray < 'd > , }
    };
}

SerializeTupleVariant!()