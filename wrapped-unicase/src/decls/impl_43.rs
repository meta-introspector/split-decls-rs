macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < S : AsRef < str > > AsRef < str > for UniCase < S > { # [inline] fn as_ref (& self) -> & str { inner ! (self . 0) . as_ref () } }
    };
}

impl_43!()