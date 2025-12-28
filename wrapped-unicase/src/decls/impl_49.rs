macro_rules! deps {
    () => {
        Encoding!();
        Ascii!();
        UniCase!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < S > From < Ascii < S > > for UniCase < S > { fn from (ascii : Ascii < S >) -> Self { UniCase (Encoding :: Ascii (ascii)) } }
    };
}

impl_49!()