macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeStruct {
    () => {
        deps!();
        struct SerializeStruct (BTreeMap < Value , Value >) ;
    };
}

SerializeStruct!();