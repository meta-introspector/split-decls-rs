macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! macro_512 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (SanitizerSet) ;
    };
}

macro_512!()