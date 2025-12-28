macro_rules! deps {
    () => {
        MirConstId!();
    };
}

macro_rules! macro_442 {
    () => {
        deps!();
        index_impl ! (MirConstId) ;
    };
}

macro_442!();