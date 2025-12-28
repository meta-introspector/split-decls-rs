macro_rules! deps {
    () => {
        BitSetExt!();
        MaybeReachable!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < S > MaybeReachable < S > { # [doc = " Return whether the current state contains the given element. If the state is unreachable,"] # [doc = " it does no contain anything."] pub fn contains < T > (& self , elem : T) -> bool where S : BitSetExt < T > , { match self { MaybeReachable :: Unreachable => false , MaybeReachable :: Reachable (set) => set . contains (elem) , } } }
    };
}

impl_89!()