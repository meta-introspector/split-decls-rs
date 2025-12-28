macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < S : FromStr + AsRef < str > > FromStr for UniCase < S > { type Err = < S as FromStr > :: Err ; fn from_str (s : & str) -> Result < UniCase < S > , Self :: Err > { s . parse () . map (UniCase :: new) } }
    };
}

impl_36!()