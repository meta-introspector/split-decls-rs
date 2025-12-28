macro_rules! deps {
    () => {
        SerializeValueArray!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        pub struct SerializeTupleVariant { variant : & 'static str , inner : SerializeValueArray , }
    };
}

SerializeTupleVariant!();