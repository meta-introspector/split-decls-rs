macro_rules! deps {
    () => {
        UniCase!();
        Ascii!();
        Encoding!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < S > From < Ascii < S > > for UniCase < S > { fn from (ascii : Ascii < S >) -> Self { UniCase (Encoding :: Ascii (ascii)) } }
    };
}

impl_22!()