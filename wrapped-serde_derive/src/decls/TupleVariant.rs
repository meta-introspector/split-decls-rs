macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! TupleVariant {
    () => {
        deps!();
        enum TupleVariant < 'a > { ExternallyTagged { type_name : & 'a Name , variant_index : u32 , variant_name : & 'a Name , } , Untagged , }
    };
}

TupleVariant!();