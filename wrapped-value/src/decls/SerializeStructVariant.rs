macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        struct SerializeStructVariant (Name , IndexMap < Name , ConstValue >) ;
    };
}

SerializeStructVariant!()