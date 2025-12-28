macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
    };
}

macro_rules! SerializeStruct {
    () => {
        deps!();
        struct SerializeStruct (IndexMap < Name , ConstValue >) ;
    };
}

SerializeStruct!();