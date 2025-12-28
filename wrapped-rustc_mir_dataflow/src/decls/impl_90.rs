macro_rules! deps {
    () => {
        BitSetExt!();
        MaybeReachable!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < T , S : BitSetExt < T > > BitSetExt < T > for MaybeReachable < S > { fn contains (& self , elem : T) -> bool { self . contains (elem) } }
    };
}

impl_90!();