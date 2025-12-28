macro_rules! deps {
    () => {
        GenKill!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T : Idx > GenKill < T > for MixedBitSet < T > { fn gen_ (& mut self , elem : T) { self . insert (elem) ; } fn kill (& mut self , elem : T) { self . remove (elem) ; } }
    };
}

impl_108!();