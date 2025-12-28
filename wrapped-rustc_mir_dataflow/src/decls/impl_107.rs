macro_rules! deps {
    () => {
        GenKill!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T : Idx > GenKill < T > for DenseBitSet < T > { fn gen_ (& mut self , elem : T) { self . insert (elem) ; } fn kill (& mut self , elem : T) { self . remove (elem) ; } }
    };
}

impl_107!();