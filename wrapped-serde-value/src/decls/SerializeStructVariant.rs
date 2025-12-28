macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        struct SerializeStructVariant (Value , BTreeMap < Value , Value >) ;
    };
}

SerializeStructVariant!()