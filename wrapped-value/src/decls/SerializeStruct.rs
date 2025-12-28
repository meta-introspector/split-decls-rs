macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! SerializeStruct {
    () => {
        deps!();
        struct SerializeStruct (IndexMap < Name , ConstValue >) ;
    };
}

SerializeStruct!()