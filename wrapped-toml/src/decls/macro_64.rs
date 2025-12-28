macro_rules! macro_64 {
    () => {
        impl_into_value ! (Integer : i32) ;
    };
}

macro_64!();