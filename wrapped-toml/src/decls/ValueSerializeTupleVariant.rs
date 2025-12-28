macro_rules! deps {
    () => {
        ValueSerializeVariant!();
        ValueSerializeVec!();
    };
}

macro_rules! ValueSerializeTupleVariant {
    () => {
        deps!();
        type ValueSerializeTupleVariant = ValueSerializeVariant < ValueSerializeVec > ;
    };
}

ValueSerializeTupleVariant!();