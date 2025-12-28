macro_rules! deps {
    () => {
        ResponseT!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < I : Interner , T > ResponseT < I > for inspect :: State < I , T > { fn var_values (& self) -> CanonicalVarValues < I > { self . var_values } }
    };
}

impl_70!()