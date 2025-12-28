macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        struct SerializeStructVariant (Name , IndexMap < Name , ConstValue >) ;
    };
}

SerializeStructVariant!();