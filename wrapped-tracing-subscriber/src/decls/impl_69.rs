macro_rules! deps {
    () => {
        DynFilterFn!();
        Context!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < F , S > From < F > for DynFilterFn < S , F > where F : Fn (& Metadata < '_ > , & Context < '_ , S >) -> bool , { fn from (f : F) -> Self { Self :: new (f) } }
    };
}

impl_69!()