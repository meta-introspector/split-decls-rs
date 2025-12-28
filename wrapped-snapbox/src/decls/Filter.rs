macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Filter {
    () => {
        deps!();
        pub trait Filter { fn filter (& self , data : Data) -> Data ; }
    };
}

Filter!()