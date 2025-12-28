macro_rules! deps {
    () => {
        LinkSelfContainedDefault!();
    };
}

macro_rules! macro_465 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (LinkSelfContainedDefault) ;
    };
}

macro_465!()