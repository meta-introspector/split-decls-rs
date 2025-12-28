macro_rules! deps {
    () => {
        SmallDataThresholdSupport!();
    };
}

macro_rules! macro_487 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (SmallDataThresholdSupport) ;
    };
}

macro_487!()