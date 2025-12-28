macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! SerializeTupleVariant {
    () => {
        deps!();
        struct SerializeTupleVariant (Name , Vec < ConstValue >) ;
    };
}

SerializeTupleVariant!()