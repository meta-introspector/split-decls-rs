macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! SerializeTupleStruct {
    () => {
        deps!();
        struct SerializeTupleStruct (Vec < ConstValue >) ;
    };
}

SerializeTupleStruct!()