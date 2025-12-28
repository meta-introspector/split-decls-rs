macro_rules! deps {
    () => {
        BitSetExt!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T : Idx > BitSetExt < T > for MixedBitSet < T > { fn contains (& self , elem : T) -> bool { self . contains (elem) } }
    };
}

impl_104!()