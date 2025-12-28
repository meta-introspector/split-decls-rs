macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! SerializeTuple {
    () => {
        deps!();
        struct SerializeTuple (Vec < ConstValue >) ;
    };
}

SerializeTuple!();