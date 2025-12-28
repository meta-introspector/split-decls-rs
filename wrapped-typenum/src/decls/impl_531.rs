macro_rules! deps {
    () => {
        TArr!();
        TypeArray!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < V , A > TypeArray for TArr < V , A > { }
    };
}

impl_531!()