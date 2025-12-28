macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < S : AsRef < str > > Eq for Ascii < S > { }
    };
}

impl_13!()