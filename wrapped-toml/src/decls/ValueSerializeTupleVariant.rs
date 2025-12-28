macro_rules! deps {
    () => {
        ValueSerializeVec!();
        ValueSerializeVariant!();
    };
}

macro_rules! ValueSerializeTupleVariant {
    () => {
        deps!();
        type ValueSerializeTupleVariant = ValueSerializeVariant < ValueSerializeVec > ;
    };
}

ValueSerializeTupleVariant!()