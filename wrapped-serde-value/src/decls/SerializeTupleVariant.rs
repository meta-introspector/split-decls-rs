macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        struct SerializeTupleVariant (Value , Vec < Value >) ;
    };
}

SerializeTupleVariant!();