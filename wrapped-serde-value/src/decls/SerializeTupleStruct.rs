macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeTupleStruct {
    () => {
        deps!();
        struct SerializeTupleStruct (Vec < Value >) ;
    };
}

SerializeTupleStruct!();