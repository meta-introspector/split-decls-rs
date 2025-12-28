macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < S : AsRef < str > > From < S > for UniCase < S > { fn from (s : S) -> Self { UniCase :: new (s) } }
    };
}

impl_52!();