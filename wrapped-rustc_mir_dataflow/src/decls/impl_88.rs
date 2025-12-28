macro_rules! deps {
    () => {
        MaybeReachable!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T > MaybeReachable < T > { pub fn is_reachable (& self) -> bool { matches ! (self , MaybeReachable :: Reachable (_)) } }
    };
}

impl_88!()