macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        impl_into_value ! (Table : Table) ;
    };
}

macro_72!();