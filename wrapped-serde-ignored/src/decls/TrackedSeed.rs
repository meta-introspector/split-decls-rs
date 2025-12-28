macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! TrackedSeed {
    () => {
        deps!();
        # [doc = " Seed used for map values, sequence elements and newtype variants to track"] # [doc = " their path."] struct TrackedSeed < 'a , X , F : 'a > { seed : X , callback : & 'a mut F , path : Path < 'a > , }
    };
}

TrackedSeed!();