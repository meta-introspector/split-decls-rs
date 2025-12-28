macro_rules! deps {
    () => {
        EndianWrapper!();
    };
}

macro_rules! macro_441 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (EndianWrapper) ;
    };
}

macro_441!()