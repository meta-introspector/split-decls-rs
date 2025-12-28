macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! macro_443 {
    () => {
        deps!();
        index_impl ! (Ty) ;
    };
}

macro_443!();