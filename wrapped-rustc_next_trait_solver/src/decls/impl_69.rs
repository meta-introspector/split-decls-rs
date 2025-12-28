macro_rules! deps {
    () => {
        ResponseT!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < I : Interner > ResponseT < I > for Response < I > { fn var_values (& self) -> CanonicalVarValues < I > { self . var_values } }
    };
}

impl_69!()