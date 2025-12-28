macro_rules! deps {
    () => {
        Interner!();
        CanonicalVarValues!();
        GenericArg!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < I : Interner > Index < ty :: BoundVar > for CanonicalVarValues < I > { type Output = I :: GenericArg ; fn index (& self , value : ty :: BoundVar) -> & I :: GenericArg { & self . var_values . as_slice () [value . as_usize ()] } }
    };
}

impl_273!()