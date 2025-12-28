macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < K , V , S > Clear for collections :: HashMap < K , V , S > where K : hash :: Hash + Eq , S : hash :: BuildHasher , { # [inline] fn clear (& mut self) { collections :: HashMap :: clear (self) } }
    };
}

impl_63!();