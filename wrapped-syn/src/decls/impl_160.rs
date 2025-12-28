macro_rules! deps {
    () => {
        TrivialDrop!();
        NoDrop!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < T > NoDrop < T > { pub (crate) fn new (value : T) -> Self where T : TrivialDrop , { NoDrop (ManuallyDrop :: new (value)) } }
    };
}

impl_160!()