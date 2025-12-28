macro_rules! deps {
    () => {
        SerializeMap!();
        Table!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl SerializeMap { pub (crate) fn new () -> Self { Self { map : Table :: new () , next_key : None , } } pub (crate) fn with_capacity (capacity : usize) -> Self { Self { map : Table :: with_capacity (capacity) , next_key : None , } } }
    };
}

impl_382!();