macro_rules! TupleTrait {
    () => {
        enum TupleTrait { SerializeTuple , SerializeTupleStruct , SerializeTupleVariant , }
    };
}

TupleTrait!();