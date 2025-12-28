macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeTuple {
    () => {
        deps!();
        struct SerializeTuple (Vec < Value >) ;
    };
}

SerializeTuple!();