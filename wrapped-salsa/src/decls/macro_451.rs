macro_rules! deps {
    () => {
        ErasedJar!();
    };
}

macro_rules! macro_451 {
    () => {
        deps!();
        # [cfg (feature = "inventory")] inventory :: collect ! (ErasedJar) ;
    };
}

macro_451!()