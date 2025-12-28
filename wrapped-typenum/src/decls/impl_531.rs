macro_rules! deps {
    () => {
        TypeArray!();
        TArr!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < V , A > TypeArray for TArr < V , A > { }
    };
}

impl_531!();