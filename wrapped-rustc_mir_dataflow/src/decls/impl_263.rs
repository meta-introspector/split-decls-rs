macro_rules! deps {
    () => {
        ValueOrPlace!();
        HasTop!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < V : HasTop > ValueOrPlace < V > { pub const TOP : Self = ValueOrPlace :: Value (V :: TOP) ; }
    };
}

impl_263!()