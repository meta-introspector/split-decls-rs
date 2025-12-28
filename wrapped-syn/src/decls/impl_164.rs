macro_rules! deps {
    () => {
        TrivialDrop!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T > TrivialDrop for iter :: Empty < T > { }
    };
}

impl_164!()