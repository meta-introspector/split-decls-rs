macro_rules! deps {
    () => {
        GenKill!();
        MaybeReachable!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T , S : GenKill < T > > GenKill < T > for MaybeReachable < S > { fn gen_ (& mut self , elem : T) { match self { MaybeReachable :: Unreachable => { } MaybeReachable :: Reachable (set) => set . gen_ (elem) , } } fn kill (& mut self , elem : T) { match self { MaybeReachable :: Unreachable => { } MaybeReachable :: Reachable (set) => set . kill (elem) , } } }
    };
}

impl_109!()