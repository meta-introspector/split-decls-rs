macro_rules! deps {
    () => {
        ExternAbiWrapper!();
    };
}

macro_rules! macro_445 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (ExternAbiWrapper) ;
    };
}

macro_445!();