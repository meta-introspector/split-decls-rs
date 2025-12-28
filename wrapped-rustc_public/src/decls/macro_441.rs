macro_rules! deps {
    () => {
        TyConstId!();
    };
}

macro_rules! macro_441 {
    () => {
        deps!();
        index_impl ! (TyConstId) ;
    };
}

macro_441!()