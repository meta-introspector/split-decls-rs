macro_rules! deps {
    () => {
        Path!();
        TrackedSeed!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a , X , F > TrackedSeed < 'a , X , F > { fn new (seed : X , callback : & 'a mut F , path : Path < 'a >) -> Self { TrackedSeed { seed , callback , path , } } }
    };
}

impl_20!()