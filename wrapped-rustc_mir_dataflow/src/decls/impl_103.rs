macro_rules! deps {
    () => {
        BitSetExt!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < T : Idx > BitSetExt < T > for DenseBitSet < T > { fn contains (& self , elem : T) -> bool { self . contains (elem) } }
    };
}

impl_103!()