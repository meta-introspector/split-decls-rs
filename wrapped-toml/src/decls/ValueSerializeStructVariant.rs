macro_rules! deps {
    () => {
        ValueSerializeMap!();
        ValueSerializeVariant!();
    };
}

macro_rules! ValueSerializeStructVariant {
    () => {
        deps!();
        type ValueSerializeStructVariant = ValueSerializeVariant < ValueSerializeMap > ;
    };
}

ValueSerializeStructVariant!()