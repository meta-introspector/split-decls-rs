macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T , S > Clear for collections :: HashSet < T , S > where T : hash :: Hash + Eq , S : hash :: BuildHasher , { # [inline] fn clear (& mut self) { collections :: HashSet :: clear (self) } }
    };
}

impl_64!();